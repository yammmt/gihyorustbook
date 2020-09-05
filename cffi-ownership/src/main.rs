use std::os::raw::{c_int, c_void};

#[link(name = "ownership", kind = "static")]
extern "C" {
    fn take_ownership(i: *const c_int, dtor: unsafe extern "C" fn(i: *mut c_int)) -> c_void;
}

#[link(name = "ownership", kind = "static")]
extern "C" {
    fn make_memory() -> *mut c_int;
}

unsafe extern "C" fn drop_pointer(i: *mut c_int) {
    // ポインタから Box に復元し, 所有権を取り戻す
    // Box の寿命がすぐに尽きるので, そこで解放される
    // Rust でアロケートしたメモリを C の `free` で解放できることは保証されていない (Rust 1.31)
    Box::from_raw(i);
}

fn main() {
    let i = Box::new(1);
    unsafe { take_ownership(Box::into_raw(i), drop_pointer)};

    unsafe {
        let i = make_memory();
        dbg!();
        println!("got {}", *i);
        // C から渡されたメモリは手動で解放する必要がある
        libc::free(i as *mut _);
    }
}
