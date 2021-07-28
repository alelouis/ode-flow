use rand::Rng;

// linear segment mapping
pub fn linear_map(value: f64, src_extent : (f64, f64), dst_extent : (f64, f64)) -> f64 {
    (((value - src_extent.0) / (src_extent.1 - src_extent.0))*(dst_extent.1 - dst_extent.0)) + dst_extent.0
}

// convert from state coordinates to pixels coordinates
pub fn state_to_pixels(
    xy: (f64, f64), 
    extent_x: (f64, f64),
    extent_y: (f64, f64),
    t_width: u32, 
    t_height: u32) -> (i32, i32) {
        let x_p = linear_map(xy.0, extent_x, (0., t_width as f64)) as i32;
        let y_p = linear_map(xy.1, extent_y, (0., t_height as f64)) as i32;
        (x_p, y_p)
    }

// computes squared norm of tuple
pub fn norm(xy: (f64, f64)) -> f64 {
    xy.0*xy.0 + xy.1*xy.1
}

// computes squared norm of tuple
pub fn generate_random_tuple(
    extent_x: (f64, f64),
    extent_y: (f64, f64),
    t_width: u32, 
    t_height: u32) -> (f64, f64) {
    let mut rng = rand::thread_rng();
    let x = linear_map(rng.gen_range(0..t_width) as f64, (0., t_width as f64), extent_x);
    let y = linear_map(rng.gen_range(0..t_height) as f64, (0., t_height as f64), extent_y);
    (x, y)
}