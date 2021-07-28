
# ODE Flow
Ordinary Differential Equation flow visualization written in Rust with SDL2.


## Ordinary Differential Equation
The ODE used is defined at the top of the `main.rs` file.
```rust
// ode func
pub fn f(x: f64, y: f64) -> (f64, f64) {
    (y, -x.sin() - 0.5 * y)
}
```
This ODE models a simple 2D pendulum with air friction.

## Flow
The particle flow is constructed by sampling randomly initial positions and propagating in gradient direction with Euler integration. A constant number of particles is kept by poping front particle vector once max size is reached.

## Example
https://user-images.githubusercontent.com/6841652/127370710-5e60a0c0-f61e-427d-8519-68e2303775f9.mov
