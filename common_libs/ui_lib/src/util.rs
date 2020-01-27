#[inline]
pub fn clamp(num: f32, min: f32, max: f32) -> f32 {
    if num < min {
        min
    } else if num > max {
        max
    } else {
        num
    }
}
