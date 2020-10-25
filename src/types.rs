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
    fn index<'a>(&'a self, i: usize) -> &'a f32 {
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

#[derive(Default)]
pub struct SH4 {
    pub band0_m0: Color,

    pub band1_m1n: Color,
    pub band1_m0: Color,
    pub band1_m1p: Color,

    pub band2_m2n: Color,
    pub band2_m1n: Color,
    pub band2_m0: Color,
    pub band2_m1p: Color,
    pub band2_m2p: Color,

    pub band3_m3n: Color,
    pub band3_m2n: Color,
    pub band3_m1n: Color,
    pub band3_m0: Color,
    pub band3_m1p: Color,
    pub band3_m2p: Color,
    pub band3_m3p: Color,
}

impl std::ops::Add<SH4> for SH4 {
    type Output = SH4;

    fn add(self, rhs: SH4) -> SH4 {
        SH4 {
            band0_m0: self.band0_m0 + rhs.band0_m0,
            band1_m1n: self.band1_m1n + rhs.band1_m1n,
            band1_m0: self.band1_m0 + rhs.band1_m0,
            band1_m1p: self.band1_m1p + rhs.band1_m1p,
            band2_m2n: self.band2_m2n + rhs.band2_m2n,
            band2_m1n: self.band2_m1n + rhs.band2_m1n,
            band2_m0: self.band2_m0 + rhs.band2_m0,
            band2_m1p: self.band2_m1p + rhs.band2_m1p,
            band2_m2p: self.band2_m2p + rhs.band2_m2p,
            band3_m3n: self.band3_m3n + rhs.band3_m3n,
            band3_m2n: self.band3_m2n + rhs.band3_m2n,
            band3_m1n: self.band3_m1n + rhs.band3_m1n,
            band3_m0: self.band3_m0 + rhs.band3_m0,
            band3_m1p: self.band3_m1p + rhs.band3_m1p,
            band3_m2p: self.band3_m2p + rhs.band3_m2p,
            band3_m3p: self.band3_m3p + rhs.band3_m3p,
        }
    }
}

impl std::ops::Div<f32> for SH4 {
    type Output = SH4;

    fn div(self, rhs: f32) -> SH4 {
        SH4 {
            band0_m0: self.band0_m0 / rhs,
            band1_m1n: self.band1_m1n / rhs,
            band1_m0: self.band1_m0 / rhs,
            band1_m1p: self.band1_m1p / rhs,
            band2_m2n: self.band2_m2n / rhs,
            band2_m1n: self.band2_m1n / rhs,
            band2_m0: self.band2_m0 / rhs,
            band2_m1p: self.band2_m1p / rhs,
            band2_m2p: self.band2_m2p / rhs,
            band3_m3n: self.band3_m3n / rhs,
            band3_m2n: self.band3_m2n / rhs,
            band3_m1n: self.band3_m1n / rhs,
            band3_m0: self.band3_m0 / rhs,
            band3_m1p: self.band3_m1p / rhs,
            band3_m2p: self.band3_m2p / rhs,
            band3_m3p: self.band3_m3p / rhs,
        }
    }
}

impl std::fmt::Display for SH4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"{{
    "band0": {{
        "m0": {}
    }},
    "band1": {{
        "m1n": {},
        "m0":  {},
        "m1p": {}
    }},
    "band2": {{
        "m2n": {},
        "m1n": {},
        "m0":  {},
        "m1p": {},
        "m2p": {}
    }},
    "band3": {{
        "m3n": {},
        "m2n": {},
        "m1n": {},
        "m0":  {},
        "m1p": {},
        "m2p": {},
        "m3p": {}
    }}
}}"#,
            self.band0_m0,
            self.band1_m1n,
            self.band1_m0,
            self.band1_m1p,
            self.band2_m2n,
            self.band2_m1n,
            self.band2_m0,
            self.band2_m1p,
            self.band2_m2p,
            self.band3_m3n,
            self.band3_m2n,
            self.band3_m1n,
            self.band3_m0,
            self.band3_m1p,
            self.band3_m2p,
            self.band3_m3p
        )
    }
}

impl SH4 {
    pub fn print_color_channel(&self, channel: usize) {
        println!(
            r#"{{
    "band0": [ {} ],
    "band1": [ {}, {}, {} ],
    "band2": [ {}, {}, {}, {}, {} ]
    "band3": [ {}, {}, {}, {}, {}, {}, {} ]
}}"#,
            self.band0_m0[channel],
            self.band1_m1n[channel],
            self.band1_m0[channel],
            self.band1_m1p[channel],
            self.band2_m2n[channel],
            self.band2_m1n[channel],
            self.band2_m0[channel],
            self.band2_m1p[channel],
            self.band2_m2p[channel],
            self.band3_m3n[channel],
            self.band3_m2n[channel],
            self.band3_m1n[channel],
            self.band3_m0[channel],
            self.band3_m1p[channel],
            self.band3_m2p[channel],
            self.band3_m3p[channel]
        )
    }
}
