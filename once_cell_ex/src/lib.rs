#[cfg(test)]
mod tests_once_cell {
    use std::collections::HashMap;
    use std::sync::Mutex;

    use once_cell::sync::Lazy;
    use once_cell::sync::OnceCell;

    #[derive(Debug)]
    struct Logger {}

    impl Logger {
        pub fn info(&self, s: &str) {
            println!("hello {}", s)
        }
    }

    static INSTANCE_LOGGER: OnceCell<Logger> = OnceCell::new();

    #[test]
    fn one_cell_it_works() {
        let l = INSTANCE_LOGGER.get_or_init(|| -> Logger { Logger {} });
        l.info("world");
    }

    #[test]
    fn one_cell_lazy() {
        static GLOBAL_DATA: Lazy<Mutex<HashMap<i32, String>>> = Lazy::new(|| {
            let mut m = HashMap::new();
            m.insert(13, "Spica".to_string());
            m.insert(74, "Hoyten".to_string());
            Mutex::new(m)
        });

        Lazy::force(&GLOBAL_DATA);
    }
}

#[cfg(test)]
mod tests_lazy_static {
    use lazy_static::lazy_static;

    // 宏生成一个唯一类型,包裹了一层实现 `Deref<TYPE>` 并绑定在静态名字上.
    // - 在里面的类型需要实现 Sync trait
    // - 如果类型有一个析构函数,那么它将不会运行
    lazy_static! {
         /// This is an example for using doc comment attributes
         static ref EXAMPLE: u8 = 42;
         static ref NUMBER: u32 = times_two(21);
    }

    fn times_two(n: u32) -> u32 {
        n * 2
    }

    #[test]
    fn test_static_macro() {
        assert_eq!(42, *EXAMPLE);
        assert_eq!(42, *NUMBER);
    }

    lazy_static! {
        static ref BUFFER: Vec<u8> = (0..255).collect();
    }
    #[test]
    fn test_static_initialize() {
        lazy_static::initialize(&BUFFER);

        println!("{:?}", *BUFFER);
    }
}
