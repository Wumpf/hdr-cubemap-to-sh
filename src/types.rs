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
            data: vec![Default::default(); (num_bands + 1) * (num_bands + 1)],
            num_bands,
        }
    }

    pub fn at(&mut self, l: i32, m: i32) -> T {
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
}
