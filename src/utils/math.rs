// this can be pretty big since the smallest value we care about is the number
// of 8th notes in a 32nd note, which is 0.25
const EPSILON: f32 = 0.001;

/// Overkill function for comparing floats
/// This is particularly overkill because of how large the epsilon is.
/// todo: think of a better way to handle comparing durations
/// [Credit](https://floating-point-gui.de/errors/comparison/)
pub fn f32_eq(a: f32, b: f32) -> bool {
    let abs_a = a.abs();
    let abs_b = b.abs();
    let diff = (a - b).abs();

    if a == b {
        true
    } else if a == 0. || b == 0. || (abs_a + abs_b < f32::MIN) {
        diff < (EPSILON * f32::MIN)
    } else {
        diff / f32::min(abs_a + abs_b, f32::MAX) < EPSILON
    }
}
