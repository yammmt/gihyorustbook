use std::os::raw::{c_double, c_int};

#[repr(C)]
pub struct point {
    x: c_int,
    y: c_int,
}

fn pow(x: c_int) -> c_int {
    x * x
}

#[no_mangle]
pub extern "C" fn dist(p1: &point, p2: &point) -> c_double {
    let d = pow(p1.x - p2.x) + pow(p1.y - p2.y);
    (d as f64).sqrt() as c_double
}
