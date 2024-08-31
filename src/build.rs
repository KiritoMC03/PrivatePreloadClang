fn main() {
    println!("start clang preload build command");
    clang_sys::load().expect("Unable to find libclang");
    let b = clang_sys::is_loaded();
    let lib = clang_sys::get_library();
    return format!("Finish clang preload build command: {}, {:?}", b, lib.unwrap().path());
