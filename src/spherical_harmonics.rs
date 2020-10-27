use itertools::Itertools;

use crate::{color::Color, mathutils::sqr};

pub struct SphericalHarmonics<T> {
    pub data: Vec<T>,
    pub num_bands: usize,
}

impl<T> std::ops::Index<usize> for SphericalHarmonics<T> {
    type Output = T;
    fn index(&self, i: usize) -> &T {
        &self.data[i]
    }
}

impl<T> std::ops::IndexMut<usize> for SphericalHarmonics<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        &mut self.data[i]
    }
}

impl std::ops::Add<SphericalHarmonics<Color>> for SphericalHarmonics<Color> {
    type Output = SphericalHarmonics<Color>;

    fn add(self, rhs: SphericalHarmonics<Color>) -> SphericalHarmonics<Color> {
        assert_eq!(rhs.num_bands, self.num_bands);
        SphericalHarmonics {
            data: self
                .data
                .iter()
                .zip(rhs.data.iter())
                .map(|v| *v.0 + *v.1)
                .collect(),
            num_bands: self.num_bands,
        }
    }
}

impl std::ops::Div<f32> for SphericalHarmonics<Color> {
    type Output = SphericalHarmonics<Color>;

    fn div(self, rhs: f32) -> SphericalHarmonics<Color> {
        SphericalHarmonics {
            data: self.data.iter().map(|c| *c / rhs).collect(),
            num_bands: self.num_bands,
        }
    }
}

impl<T: Clone + Copy + Default + std::fmt::Display> SphericalHarmonics<T> {
    pub fn new(num_bands: usize) -> Self {
        SphericalHarmonics {
            data: vec![Default::default(); sqr(num_bands + 1)],
            num_bands,
        }
    }

    pub fn at(&self, l: i32, m: i32) -> T {
        self.data[(l * l + l + m) as usize]
    }

    pub fn at_mut(&mut self, l: i32, m: i32) -> &T {
        &mut self.data[(l * l + l + m) as usize]
    }

    pub fn band(&self, l: usize) -> &[T] {
        let band_start = l * l + l;
        let num_in_band = l * 2 + 1;
        &self.data[band_start..(band_start + num_in_band)]
    }

    pub fn print(&self) {
        println!(
            "{{\n{}\n}}",
            (0..self.num_bands)
                .map(|l| {
                    format!(
                        "    band{}: [ {} ]",
                        l,
                        self.band(l).iter().map(|v| v.to_string()).join(", ")
                    )
                })
                .join("\n")
        );
    }
}

impl SphericalHarmonics<Color> {
    pub fn print_color_channel(&self, channel: usize) {
        println!(
            "{{\n{}\n}}",
            (0..self.num_bands)
                .map(|l| {
                    format!(
                        "    band{}: [ {} ]",
                        l,
                        self.band(l).iter().map(|v| v[channel]).join(", ")
                    )
                })
                .join("\n")
        );
    }

    pub fn luminance(&self) -> SphericalHarmonics<f32> {
        SphericalHarmonics::<f32> {
            data: self.data.iter().map(|c| c.luminance()).collect(),
            num_bands: self.num_bands,
        }
    }
}
