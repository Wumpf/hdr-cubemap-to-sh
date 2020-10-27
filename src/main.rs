use color::*;
use image::hdr::HdrDecoder;
use mathutils::*;
use spherical_harmonics::*;
use std::{fs::File, io::BufReader, path::Path, thread::JoinHandle};

mod color;
mod mathutils;
mod spherical_harmonics;

const NUM_BANDS: usize = 3;

fn main() {
    const USAGE: &'static str =  "Tool must be invoked with path to a folder containing cubemap .hdr (square) pictures in the form px.hdr, nx.hdr, py.hdr, ny.hdr, pz.hdr, nz.hdr ";

    let argument = std::env::args().nth(1).expect(USAGE);
    let path = Path::new(&argument);
    if !path.is_dir() {
        panic!("Passed path {:?} is not a directory", path)
    }

    // I am a simple coder. I see 6 files that I can process, I create 6 threads, I wait for them to finish.
    // The workload isn't all that fancy/big to warrant a job system but not doing it parallel hurts my soul as well.
    let filenames = ["px.hdr", "nx.hdr", "py.hdr", "ny.hdr", "pz.hdr", "nz.hdr"];
    let file_processor_threads: Vec<JoinHandle<SphericalHarmonics<Color>>> = filenames
        .iter()
        .enumerate()
        .map(|(face_idx, filename)| {
            let filepath = path.join(filename);
            std::thread::spawn(move || compute_sh_for_side(face_idx, filepath))
        })
        .collect();
    let sh: SphericalHarmonics<Color> = file_processor_threads
        .into_iter()
        .map(|thread| thread.join().expect("Failed to process file"))
        .fold(SphericalHarmonics::new(NUM_BANDS), |a, b| a + b); // All samples are weighted with steradian, so we can just add!

    sh.print();
    println!();
    println!();
    // println!("color by color");
    // println!("red:");
    // sh.print_color_channel(0);
    // println!("blue:");
    // sh.print_color_channel(1);
    // println!("green:");
    // sh.print_color_channel(2);
    // println!();
    // println!();
    let max_laplacian = 1.0;
    println!("computing windowed SH with max_laplacian {}...", max_laplacian);

    let sh_luminance = sh.luminance();
    let lambda = find_windowing_lambda(&sh_luminance, max_laplacian);
    println!("lambda: {}", lambda);
    let windowed_sh = apply_windowing(&sh, lambda);
    windowed_sh.print();
}

fn compute_sh_for_side(face_idx: usize, path: std::path::PathBuf) -> SphericalHarmonics<Color> {
    println!("Processing {:?} (face index {})..", path, face_idx);

    let file_reader = BufReader::new(File::open(&path).unwrap());
    let decoder = HdrDecoder::new(file_reader).unwrap();
    let metadata = decoder.metadata();
    if metadata.height != metadata.width {
        panic!("cubemap face width not equal height");
    }

    let image_data = decoder.read_image_hdr().unwrap();
    let inv_size = 1.0 / (metadata.width as f32);
    let mut sh = SphericalHarmonics::new(NUM_BANDS);

    for (v, row) in image_data.chunks(metadata.width as usize).enumerate() {
        for (u, &pixel) in row.iter().enumerate() {
            let weight = texel_coord_solid_angle(u, v, inv_size);

            let dir: (f32, f32, f32) = match face_idx {
                // Positive X
                0 => (1.0 - (2.0 * u as f32 + 1.0) * inv_size, 1.0 - (2.0 * v as f32 + 1.0) * inv_size, 1.0),
                // Negative X
                1 => (-1.0 + (2.0 * u as f32 + 1.0) * inv_size, 1.0 - (2.0 * v as f32 + 1.0) * inv_size, -1.0),
                // Positive Y
                2 => (-1.0 + (2.0 * v as f32 + 1.0) * inv_size, 1.0, -1.0 + (2.0 * u as f32 + 1.0) * inv_size),
                // Negative Y
                3 => (1.0 - (2.0 * v as f32 + 1.0) * inv_size, -1.0, -1.0 + (2.0 * u as f32 + 1.0) * inv_size),
                // Positive Z
                4 => (1.0, 1.0 - (2.0 * v as f32 + 1.0) * inv_size, -1.0 + (2.0 * u as f32 + 1.0) * inv_size),
                // Negative Z
                5 => (-1.0, 1.0 - (2.0 * v as f32 + 1.0) * inv_size, 1.0 - (2.0 * u as f32 + 1.0) * inv_size),
                _ => panic!("invalid face index"),
            };
            // (yes this is written in a brute force manner and yes there's more length calc in texel_coord_solid_angle)
            let dir_len = (sqr(dir.0) + sqr(dir.1) + sqr(dir.2)).sqrt();
            let dir = (dir.0 / dir_len, dir.1 / dir_len, dir.2 / dir_len);

            let pixel_color = Color {
                r: pixel[0],
                g: pixel[1],
                b: pixel[2],
            };
            add_sample(&mut sh, dir, pixel_color, weight);
        }
    }

    println!("{:?} done..", path);

    sh / (2.0 * std::f32::consts::TAU)
}

fn add_sample(sh: &mut SphericalHarmonics<Color>, dir: (f32, f32, f32), pixel_color: Color, weight: f32) {
    // Via "Stupid Spherical Harmonics(SH) Tricks", Appendix A1
    // (can't do sqrt on const in Rust)
    let sh_basis_factor_band0 = (1.0 / (2.0 * std::f64::consts::PI.sqrt())) as f32;
    let sh_basis_factor_band1 = (3.0_f64.sqrt() / (2.0 * std::f64::consts::PI.sqrt())) as f32;
    let sh_basis_factor_band2_non0 = (15.0_f64.sqrt() / (2.0 * std::f64::consts::PI.sqrt())) as f32;
    let sh_basis_factor_band2_0 = (5.0_f64.sqrt() / (4.0 * std::f64::consts::PI.sqrt())) as f32;
    let sh_basis_factor_band3_3 = (70.0_f64.sqrt() / (8.0 * std::f64::consts::PI.sqrt())) as f32;
    let sh_basis_factor_band3_2 = (105.0_f64.sqrt() / (2.0 * std::f64::consts::PI.sqrt())) as f32;
    let sh_basis_factor_band3_1 = (42.0_f64.sqrt() / (8.0 * std::f64::consts::PI.sqrt())) as f32;
    let sh_basis_factor_band3_0 = (7.0_f64.sqrt() / (4.0 * std::f64::consts::PI.sqrt())) as f32;

    sh[0] += pixel_color * (weight * sh_basis_factor_band0);

    if NUM_BANDS > 0 {
        sh[1] += pixel_color * (-weight * sh_basis_factor_band1 * dir.1);
        sh[2] += pixel_color * (weight * sh_basis_factor_band1 * dir.2);
        sh[3] += pixel_color * (-weight * sh_basis_factor_band1 * dir.0);
    }

    if NUM_BANDS > 1 {
        sh[4] += pixel_color * (weight * sh_basis_factor_band2_non0 * dir.1 * dir.0);
        sh[5] += pixel_color * (-weight * sh_basis_factor_band2_non0 * dir.1 * dir.2);
        sh[6] += pixel_color * (weight * sh_basis_factor_band2_0 * (3.0 * sqr(dir.2) - 1.0));
        sh[7] += pixel_color * (-weight * sh_basis_factor_band2_non0 * dir.0 * dir.2);
        sh[8] += pixel_color * (weight * sh_basis_factor_band2_non0 * (sqr(dir.0) - sqr(dir.1)) * 0.5);
    }

    if NUM_BANDS > 2 {
        sh[9] += pixel_color * (-weight * sh_basis_factor_band3_3 * dir.1 * (3.0 * sqr(dir.0) - sqr(dir.1)));
        sh[10] += pixel_color * (weight * sh_basis_factor_band3_2 * dir.0 * dir.1 * dir.2);
        sh[11] += pixel_color * (-weight * sh_basis_factor_band3_1 * dir.1 * (5.0 * sqr(dir.2) - 1.0));
        sh[12] += pixel_color * (weight * sh_basis_factor_band3_0 * dir.2 * (5.0 * sqr(dir.2) - 3.0));
        sh[13] += pixel_color * (-weight * sh_basis_factor_band3_1 * dir.0 * (5.0 * sqr(dir.2) - 1.0));
        sh[14] += pixel_color * (weight * sh_basis_factor_band3_2 * (0.5 * (sqr(dir.0) - sqr(dir.1)) * dir.2));
        sh[15] += pixel_color * (-weight * sh_basis_factor_band3_3 * dir.0 * (sqr(dir.0) - 3.0 * sqr(dir.1)));
    }

    if NUM_BANDS > 3 {
        unimplemented!();
    }
}

// --------------------------------------------------------------------------------------------------------------
// Adapted from https://github.com/kayru/Probulator/blob/master/Source/Probulator/SphericalHarmonics.h

fn find_windowing_lambda(sh: &SphericalHarmonics<f32>, max_laplacian: f32) -> f32 {
    // http://www.ppsloan.org/publications/StupidSH36.pdf
    // Appendix A7 Solving for Lambda to Reduce the Squared Laplacian

    let mut table_l = vec![0.0; sh.num_bands + 1];
    let mut table_b = vec![0.0; sh.num_bands + 1];

    table_l[0] = 0.0;
    table_b[0] = 0.0;

    for l in 1..=sh.num_bands {
        table_l[l] = (sqr(sh.num_bands) * sqr(sh.num_bands + 1)) as f32;

        for m in -1..=l as i32 {
            let v = sh.at(l as i32, m);
            table_b[l] += sqr(v);
        }
    }

    let mut squared_laplacian = 0.0;
    for l in 1..=sh.num_bands {
        squared_laplacian += table_l[l] * table_b[l];
    }

    let target_squared_laplacian = max_laplacian * max_laplacian;
    if squared_laplacian <= target_squared_laplacian {
        return 0.0;
    }

    let mut lambda = 0.0;

    let iteration_limit = 10000000;
    for _ in 0..iteration_limit {
        let mut f = 0.0;
        let mut fd = 0.0;

        for l in 1..=sh.num_bands {
            f += table_l[l] * table_b[l] / sqr(1.0 + lambda * table_l[l]);
            fd += (2.0 * sqr(table_l[l]) * table_b[l]) / cube(1.0 + lambda * table_l[l]);
        }

        f = target_squared_laplacian - f;

        let delta = -f / fd;
        lambda += delta;

        if delta.abs() < 1e-6 {
            break;
        }
    }

    lambda
}

fn apply_windowing(sh: &SphericalHarmonics<Color>, lambda: f32) -> SphericalHarmonics<Color> {
    // From Peter-Pike Sloan's Stupid SH Tricks
    // http://www.ppsloan.org/publications/StupidSH36.pdf
    let mut windowed_sh = SphericalHarmonics::new(sh.num_bands);

    let mut i = 0;
    for l in 0..=sh.num_bands as i32 {
        let s = 1.0 / (1.0 + lambda * sqr(l) as f32 * sqr(l as f32 + 1.0));
        for _m in (-l)..=l {
            windowed_sh[i] = sh[i] * s;
            i += 1;
        }
    }

    windowed_sh
}

// --------------------------------------------------------------------------------------------------------------
// From http://www.rorydriscoll.com/2012/01/15/cubemap-texel-solid-angle/ / AMD CubeMapGen

fn area_element(x: f32, y: f32) -> f32 {
    (x * y).atan2((x * x + y * y + 1.0).sqrt())
}
fn texel_coord_solid_angle(u: usize, v: usize, inv_size: f32) -> f32 {
    //scale up to [-1, 1] range (inclusive), offset by 0.5 to point to texel center.
    let u: f32 = (2.0 * (u as f32 + 0.5) * inv_size) - 1.0;
    let v: f32 = (2.0 * (v as f32 + 0.5) * inv_size) - 1.0;

    // U and V are the -1..1 texture coordinate on the current face.
    // Get projected area for this texel
    let x0 = u - inv_size;
    let y0 = v - inv_size;
    let x1 = u + inv_size;
    let y1 = v + inv_size;
    let solid_angle = area_element(x0, y0) - area_element(x0, y1) - area_element(x1, y0) + area_element(x1, y1);

    return solid_angle;
}
