pub fn animating(current: f32, target: f32, rate: f32, epsilon: f32) -> (f32, bool) {
    let diff = target - current;
    if diff.abs() < epsilon {
        (target, true)
    } else {
        (current + diff * rate, false)
    }
}
