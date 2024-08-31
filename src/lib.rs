pub fn load() -> String {
    println!("start");
    clang_sys::load().expect("Unable to find libclang");
    let b = clang_sys::is_loaded();
    let lib = clang_sys::get_library();
    return format!("Finish: {}, {:?}", b, lib.unwrap().path());
}