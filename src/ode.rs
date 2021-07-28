// pendulum with friction
pub fn pendulum(x: f64, y: f64) -> (f64, f64) {
    (y, -x.sin() - 0.5 * y)
}

