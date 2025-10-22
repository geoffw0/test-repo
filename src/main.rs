
mod defs;
use defs::myMacro;
use reqwest;

fn my_function() {
    println!("Hello from my_function.");
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
    let password = "123456";
    println!("logging in (password is: {password})");

    // use of HTTP
    let page_data: String = reqwest::blocking::get("http://example.com/2/").unwrap().text().unwrap();
    println!("web data = {page_data}");

    // weak hashing
    let digest = format!("{:x}", md5::compute(password));
    println!("digest = {digest}");

    // uncontrolled allocation size
    let size = page_data.parse::<usize>().unwrap_or(1024);
    println!("size = {size}");
    let layout = std::alloc::Layout::from_size_align(size + 16, 8).unwrap();

    unsafe {
        let ptr = std::alloc::alloc(layout);

        // access after deallocation
    	std::alloc::dealloc(ptr, layout);

        let data = *ptr;
        println!("data = {data}");
    }

    println!("end.");
}
