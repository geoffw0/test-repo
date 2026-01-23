
mod defs;
use defs::myMacro;
use reqwest;
use aes_gcm::{Aes256Gcm, KeyInit};

fn my_function() {
    println!("Hello from my_function.");
}

fn set_password(password: &str) {
    let _password = password;
    // ...
}

// Hard-coded cryptographic value example
fn hardcoded_crypto_example() {
    // BAD: Using hard-coded key for encryption
    let key: [u8; 32] = [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
        0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10,
        0x0f, 0x1e, 0x2d, 0x3c, 0x4b, 0x5a, 0x69, 0x78,
        0x87, 0x96, 0xa5, 0xb4, 0xc3, 0xd2, 0xe1, 0xf0,
    ]; // Hard-coded key
    let _cipher = Aes256Gcm::new(&key.into());
    println!("Created cipher with hard-coded key (BAD practice!)");
}

// conditional compilation
#[cfg(false)]
fn not_compiled_function() {
    println!("Hello from not_compiled_function.");
    my_function();
    my_function();
    my_function();
    my_function();
    my_function();
    myMacro!();
    myMacro!();
    myMacro!();
    myMacro!();
    myMacro!();
}

fn main() {
    println!("Hello from main.");
    my_function();
    myMacro!();

    // cleartext logging
    let password2 = "123456";
    println!("logging in (password is: {password2})");

    // use of HTTP
    let page_data = reqwest::blocking::get("http://example.com/2/").unwrap().text().unwrap();
    println!("web data = {page_data}");

    // weak hashing
    let digest = format!("{:x}", md5::compute(password2));
    println!("digest = {digest}");

    // uncontrolled allocation size
    let size = page_data.parse::<usize>().unwrap_or(1024);
    println!("size = {size}");
    let layout = std::alloc::Layout::from_size_align(size + 2, 1).unwrap();

    unsafe {
        let ptr = std::alloc::alloc(layout);

        // access after deallocation
    	std::alloc::dealloc(ptr, layout);

        let data2 = *ptr;
        println!("data2 = {data2}");
    }

    // constant password
    let pwd = "123456";
    set_password(pwd);

    // hard-coded cryptographic value
    hardcoded_crypto_example();

    println!("end.");
}
