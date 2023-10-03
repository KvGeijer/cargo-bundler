mod simple {
    mod internal {
        pub fn hello_world() {
            println!("Hello, world!");
        }
    }
    pub use internal::hello_world;
}
fn main() {
    simple::hello_world();
}
