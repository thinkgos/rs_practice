#[cfg(test)]
mod tests {
    use typed_builder::TypedBuilder;

    #[derive(TypedBuilder)]
    struct Foo {
        // Mandatory Field:
        x: i32,

        // #[builder(default)] without parameter - use the type's default
        // #[builder(setter(strip_option))] - wrap the setter argument with `Some(...)`
        #[builder(default, setter(strip_option))]
        y: Option<i32>,

        // Or you can set the default
        #[builder(default = 20)]
        z: i32,
    }

    #[test]
    fn typed_builder_works() {
        let a = Foo::builder().y(1).x(1).build();
    }
}
