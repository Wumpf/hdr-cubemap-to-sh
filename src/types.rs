#[derive(Default)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl std::ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl std::ops::Div<f32> for Color {
    type Output = Color;

    fn div(self, rhs: f32) -> Color {
        Color {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}
impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.r, self.g, self.b)
    }
}

#[derive(Default)]
pub struct SH3 {
    band0_m0: Color,

    band1_m1n: Color,
    band1_m0: Color,
    band1_m1p: Color,

    band2_m2n: Color,
    band2_m1n: Color,
    band2_m0: Color,
    band2_m1p: Color,
    band2_m2p: Color,
}

impl std::ops::Add<SH3> for SH3 {
    type Output = SH3;

    fn add(self, rhs: SH3) -> SH3 {
        SH3 {
            band0_m0: self.band0_m0 + rhs.band0_m0,
            band1_m1n: self.band1_m1n + rhs.band1_m1n,
            band1_m0: self.band1_m0 + rhs.band1_m0,
            band1_m1p: self.band1_m1p + rhs.band1_m1p,
            band2_m2n: self.band2_m2n + rhs.band2_m2n,
            band2_m1n: self.band2_m1n + rhs.band2_m1n,
            band2_m0: self.band2_m0 + rhs.band2_m0,
            band2_m1p: self.band2_m1p + rhs.band2_m1p,
            band2_m2p: self.band2_m2p + rhs.band2_m2p,
        }
    }
}

impl std::ops::Div<f32> for SH3 {
    type Output = SH3;

    fn div(self, rhs: f32) -> SH3 {
        SH3 {
            band0_m0: self.band0_m0 / rhs,
            band1_m1n: self.band1_m1n / rhs,
            band1_m0: self.band1_m0 / rhs,
            band1_m1p: self.band1_m1p / rhs,
            band2_m2n: self.band2_m2n / rhs,
            band2_m1n: self.band2_m1n / rhs,
            band2_m0: self.band2_m0 / rhs,
            band2_m1p: self.band2_m1p / rhs,
            band2_m2p: self.band2_m2p / rhs,
        }
    }
}

impl std::fmt::Display for SH3 {
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
            self.band2_m2p
        )
    }
}
