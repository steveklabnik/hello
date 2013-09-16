#[link(name = "hello",
       vers = "0.1.0",
       uuid = "0028fbe0-1f1f-11e3-8224-0800200c9a66",
       url = "https://github.com/steveklabnik/hello")]; 

#[desc = "A hello world Rust package."];
#[license = "MIT"];
#[crate_type = "lib"];

pub fn world() {
    println("Hello, world.");
}
