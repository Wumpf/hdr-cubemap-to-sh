use std::{
    path::{Path, PathBuf},
    thread::JoinHandle,
};
use types::*;

mod types;

fn main() {
    const USAGE: &'static str =  "Tool must be invoked with path to a folder containing cubemap .hdr pictures in the form px.hdr, nx.hdr, py.hdr, ny.hdr, pz.hdr, nz.hdr ";

    let argument = std::env::args().nth(1).expect(USAGE);
    let path = Path::new(&argument);
    if !path.is_dir() {
        panic!("Passed path {:?} is not a directory", path)
    }

    // I am a simple coder. I see 6 files that I can process, I create 6 threads, I wait for them to finish.
    // The workload isn't all that fancy/big to warrant a job system but not doing it parallel hurts my soul as well.
    let filenames = ["px.hdr", "nx.hdr", "py.hdr", "ny.hdr", "pz.hdr", "nz.hdr"];
    let file_processor_threads: Vec<JoinHandle<SH3>> = filenames
        .iter()
        .map(|filename| {
            let filepath = path.join(filename);
            std::thread::spawn(move || compute_sh_for_side(filepath))
        })
        .collect();
    let sh: SH3 = file_processor_threads
        .into_iter()
        .map(|thread| thread.join().expect("Failed to process file"))
        .fold(SH3::default(), |a, b| a + b)
        / 6.0;

    print!("{}", sh);
}

fn compute_sh_for_side(path: std::path::PathBuf) -> SH3 {
    let sh = SH3::default();

    sh
}
