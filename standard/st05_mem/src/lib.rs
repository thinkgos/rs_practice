use std::mem;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct TestData {
    name: String,
    gender: u8,
    age: u32,
}

impl TestData {
    /// 获取设置结构体名字为空, 并获取名字和其所有权,
    fn get_and_name(&mut self) -> String {
        // 不能获取引用字段的所有权
        // 可以改变字段的值, 但不能获取字段的所有权
        // 可以采用 mem::take, mem::replace
        // let b = self.name;
        // self.name = "".to_owned();
        // b
        mem::take(&mut self.name)
    }
    fn get_name(&mut self) -> String {
        mem::replace(&mut self.name, "Jim".to_owned())
    }
}

enum Foo {
    A(&'static str),
    B(i32),
    C(i32),
}

#[cfg(test)]
mod tests {
    use crate::Foo;
    use crate::TestData;
    use std::mem;

    #[test]
    fn sized_of() {
        assert_eq!(4, mem::size_of::<f32>());
        assert_eq!(32, mem::size_of::<TestData>());

        let test_data = &TestData::default();
        assert_eq!(32, mem::size_of_val(test_data));
    }
    #[test]
    fn align_of() {
        assert_eq!(8, mem::align_of::<TestData>());

        let test_data = &TestData::default();
        assert_eq!(8, mem::align_of_val(test_data));
    }
    #[test]
    fn replace() {
        let mut v1 = TestData {
            name: "Tom".to_owned(),
            gender: 1,
            age: 28,
        };
        let v2 = TestData {
            name: "LiLi".to_owned(),
            gender: 2,
            age: 32,
        };
        // v1将被v2替代, 并且返回 先前的 v1 给 old_v1
        let old_v1 = mem::replace(&mut v1, v2.clone());
        assert_eq!(
            old_v1,
            TestData {
                name: "Tom".to_owned(),
                gender: 1,
                age: 28,
            }
        );
        assert_eq!(v1, v2);
    }
    #[test]
    fn take() {
        let mut b1 = "taker";
        let old_b1 = mem::take(&mut b1);
        assert!(b1.is_empty());
        assert_eq!(old_b1, "taker");
    }
    #[test]
    fn swap() {
        let mut a1 = "hello";
        let mut a2 = "world";
        mem::swap(&mut a1, &mut a2);
        assert_eq!(a1, "world");
        assert_eq!(a2, "hello");

        // test take and replace in struct with reference
        let mut vt = TestData {
            name: "Tom".to_owned(),
            gender: 2,
            age: 100,
        };
        let name1 = vt.get_name();
        let name2 = vt.get_and_name();
        assert_eq!(name1, "Tom");
        assert_eq!(name2, "Jim",);
        assert!(vt.name.is_empty());
    }

    #[test]
    fn discriminant() {
        // discriminant 枚举识别位
        assert_eq!(
            mem::discriminant(&Foo::A("bar")),
            mem::discriminant(&Foo::A("baz"))
        );
        assert_eq!(mem::discriminant(&Foo::B(1)), mem::discriminant(&Foo::B(2)));
        assert_ne!(mem::discriminant(&Foo::B(3)), mem::discriminant(&Foo::C(3)));
    }
    // TODO: 其它一些函数, 如 needs_drop, forget, transmute⚠
    // https://doc.rust-lang.org/std/mem/index.html
}
