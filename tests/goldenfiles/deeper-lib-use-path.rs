mod deeper_lib_use_path {
    pub mod internal {
        pub fn hello_world() {
            println!("Hello, world!");
        }
        pub mod deeper_lib_use_path {
            pub fn test() {
                println!("Test");
            }
        }
    }
}
use deeper_lib_use_path::internal::hello_world;
fn main() {
    hello_world();
    bin_internal::hello();
}
mod bin_internal {
    use crate::deeper_lib_use_path;
    use crate::deeper_lib_use_path::internal::hello_world as hello_lib;
    use deeper_lib_use_path::internal;
    use deeper_lib_use_path::internal::hello_world;
    pub fn hello() {
        hello_lib();
        hello_world();
        internal::deeper_lib_use_path::test();
    }
}
