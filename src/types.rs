use itertools::Itertools;

#[derive(Default, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl std::ops::Add<Color> for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}
impl std::ops::AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}
impl std::ops::Div<f32> for Color {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Color {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}
impl std::ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}
impl std::ops::Index<usize> for Color {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 {
        match i {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            _ => panic!("invalid color index"),
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.r, self.g, self.b)
    }
}

pub struct SphericalHarmonics {
    pub data: Vec<Color>,
    pub num_bands: usize,
}

impl std::ops::Index<usize> for SphericalHarmonics {
    type Output = Color;
    fn index(&self, i: usize) -> &Color {
        &self.data[i]
    }
}

impl std::ops::IndexMut<usize> for SphericalHarmonics {
    fn index_mut(&mut self, i: usize) -> &mut Color {
        &mut self.data[i]
    }
}

impl std::ops::Add<SphericalHarmonics> for SphericalHarmonics {
    type Output = SphericalHarmonics;

    fn add(self, rhs: SphericalHarmonics) -> SphericalHarmonics {
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

impl std::ops::Div<f32> for SphericalHarmonics {
    type Output = SphericalHarmonics;

    fn div(self, rhs: f32) -> SphericalHarmonics {
        SphericalHarmonics {
            data: self.data.iter().map(|c| *c / rhs).collect(),
            num_bands: self.num_bands,
        }
    }
}

impl SphericalHarmonics {
    pub fn new(num_bands: usize) -> Self {
        SphericalHarmonics {
            data: vec![Default::default(); (num_bands + 1) * (num_bands + 1)],
            num_bands,
        }
    }

    pub fn at(&mut self, l: i32, m: i32) -> Color {
        self.data[(l * l + l + m) as usize]
    }

    pub fn at_mut(&mut self, l: i32, m: i32) -> &Color {
        &mut self.data[(l * l + l + m) as usize]
    }

    pub fn band(&self, l: usize) -> &[Color] {
        let band_start = l * l + l;
        let num_in_band = l * 2 + 1;
        &self.data[band_start..(band_start + num_in_band)]
    }

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
