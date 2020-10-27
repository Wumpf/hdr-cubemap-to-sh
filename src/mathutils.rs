pub fn sqr<T: std::ops::Mul<Output = T> + Copy>(v: T) -> T {
    v * v
}

pub fn cube<T: std::ops::Mul<Output = T> + Copy>(v: T) -> T {
    v * v * v
}
