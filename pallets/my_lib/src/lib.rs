
pub mod my_lib {
    use my_lib_to_import::my_lib_to::MyLibTo;
    pub struct MyLib {
        pub mlt: MyLibTo,
    }

    impl MyLib {
        pub fn new() -> Self {
            let mlt = MyLibTo::new();
            Self {
                mlt
            }
        }

        pub fn get(&self) -> MyLibTo {
            self.mlt.clone()
        }

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
