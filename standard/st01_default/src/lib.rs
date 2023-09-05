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
}
