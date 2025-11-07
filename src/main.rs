
mod defs;
use defs::myMacro;
use reqwest;

fn my_function() {
    println!("Hello from my_function.");
}

fn set_password(password: &str) {
    let _password = password;
    // ...
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

    println!("end.");
}
