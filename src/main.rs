
mod defs;
use defs::myMacro;
use reqwest;
use md5::{Digest};

fn main() {
    // use some macros
    println!("Hello, world!");
    myMacro!();

    // cleartext logging
    let password = "123456";
    println!("logging in (password is: {password})");

    // use of HTTP
    let page_data = reqwest::blocking::get("http://example.com/").unwrap().text().unwrap();
    println!("web data = {page_data}");

    // weak hashing
    let digest = format!("{:x}", md5::Md5::digest(password));
    println!("digest = {digest}");

    // uncontrolled allocation size
    let size = digest.to_string().parse::<usize>().unwrap_or(1024);
    println!("size = {size}");
    let layout = std::alloc::Layout::from_size_align(size, 1).unwrap();

    unsafe {
        let ptr = std::alloc::alloc(layout);

        // access after deallocation
    	std::alloc::dealloc(ptr, layout);

        let data = *ptr;
        println!("data = {data}");
    }
}
