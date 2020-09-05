use std::os::raw::c_int;

#[link(name = "readline")]
extern "C" {
    static rl_readline_version: c_int;
}

fn main() {
    unsafe {
        println!("using readline version {:x}", rl_readline_version);
    }
}
