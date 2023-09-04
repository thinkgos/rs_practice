#[derive(Debug, Default, PartialEq, Eq)]
struct A {
    name: String,
    age: Option<i32>,
}

#[cfg(test)]
mod tests {
    use crate::A;

    #[test]
    fn type_a_default() {
        assert_eq!(
            A {
                name: "li li".to_owned(),
                ..A::default()
            },
            A {
                name: "li li".to_owned(),
                age: None,
            }
        );
    }

    #[test]
    fn trait_a_default() {
        // 可以使用 default::default() 但只支持 nightly 版本.
        // 最好使用 T::default() 方法
        #![cfg(default_free_fn)]
        assert_eq!(
            A {
                name: "ming ming".to_owned(),
                ..std::default::default()
            },
            A {
                name: "ming ming".to_owned(),
                age: None,
            }
        );
    }
}
