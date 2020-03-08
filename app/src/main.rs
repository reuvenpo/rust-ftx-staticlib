
#[link(name = "enclave_check", kind = "static")]
extern "C" {
    fn x_add(x: i32, y: i32) -> i32;
}

fn safe_add(x: i32, y: i32) -> i32 {
    unsafe { x_add(x, y) }
}

fn main() {
    println!("the enclave returned: {}!", safe_add(3, 4));
}
