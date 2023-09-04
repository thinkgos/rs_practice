#[cfg(test)]
mod tests {
    /// 该模块提供了计算值哈希的通用方法。哈希最常用于  HashMap 和 HashSet。
    /// 最简单的方法就是 使用 #[derive(Hash)]
    ///
    /// 模块包含三个 trait:
    /// 1. Hash trait
    /// 类型实现了 Hash trait 那么 表示可以使用 Hasher 对其进行 hashed. 可使用 #[derive(Hash)] 实现.
    ///
    /// 2. Hasher trait
    /// 哈希任意字节的 trait.
    /// Hasher提供了一个相当基本的接口，用于检索生成的哈希.
    ///
    /// 3. BuildHasher trait
    /// 创建 Hasher 实例的 trait.
    ///
    use std::collections::hash_map::DefaultHasher;
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hash, Hasher};

    #[derive(Hash)]
    struct Person {
        id: u32,
        name: String,
        phone: u64,
    }

    #[test]
    fn hash() {
        let person1 = Person {
            id: 5,
            name: "Janet".to_string(),
            phone: 555_666_7777,
        };
        let person2 = Person {
            id: 5,
            name: "Bob".to_string(),
            phone: 555_666_7777,
        };
        fn calculate_hash<T: Hash>(t: &T) -> u64 {
            let mut s = DefaultHasher::new();
            t.hash(&mut s);
            s.finish()
        }
        assert!(calculate_hash(&person1) != calculate_hash(&person2));
    }
    #[test]
    fn builder_hash1() {
        let person1 = Person {
            id: 5,
            name: "Janet".to_string(),
            phone: 555_666_7777,
        };
        let person2 = Person {
            id: 5,
            name: "Janet".to_string(),
            phone: 555_666_7777,
        };

        let s = RandomState::new();
        let mut hasher_1 = s.build_hasher();
        let mut hasher_2 = s.build_hasher();

        person1.hash(&mut hasher_1);
        person2.hash(&mut hasher_2);

        assert_eq!(hasher_1.finish(), hasher_2.finish());
    }

    #[test]
    fn build_hasher2() {
        let s = RandomState::new();
        let mut hasher_1 = s.build_hasher();
        let mut hasher_2 = s.build_hasher();

        hasher_1.write_u32(8128);
        hasher_2.write_u32(8128);

        assert_eq!(hasher_1.finish(), hasher_2.finish());
    }
}
