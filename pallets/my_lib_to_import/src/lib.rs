
pub mod my_lib_to {
    #[derive(Clone)]
    pub struct MyLibTo {
        pub x: u32,
    }

    impl MyLibTo {
        pub fn new() -> Self {
            let x = 0;
            Self { x }
        }
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn it_works() {
            let mlt = super::MyLibTo::new();
            println!("x = {}", mlt.x);
            assert_eq!(2 + 2, 4);
        }
    }    
}

