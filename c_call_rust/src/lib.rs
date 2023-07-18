#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct point {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}

pub type Point = point;
#[no_mangle]
pub extern "C" fn print_point(x: i32, y: i32, obj: &mut Point) {
    obj.x = x;
    obj.y = y;
    println!("{} {}", obj.x, obj.y);
}