#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CoolStruct {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}

extern "C" {
    pub fn cool_function(
        i: ::std::os::raw::c_int, 
        c: ::std::os::raw::c_char, 
        cs: *mut CoolStruct);
}

fn main() {
    let mut obj = CoolStruct {
        x: 0,
        y: 0,
    };
    unsafe {cool_function(1, 50, &mut obj)};
    println!("{} {}", obj.x, obj.y);
}