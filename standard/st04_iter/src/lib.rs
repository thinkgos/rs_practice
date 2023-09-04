///
/// # 迭代器模块:
///     如果您发现自己有某种集合，并且需要对所述元素的集合进行操作，那么您将很快遇到“迭代器”.
/// # 组织架构:
///    - Traits 是核心部分: 这些特征定义了存在什么样的迭代器以及您可以使用它们做什么. 这些特征的方法值得花一些额外的学习时间。
///    - Functions 提供一些有用的方法来创建一些基本的迭代器.
///    - Structs 通常是该模块的 traits 上各种方法的返回类型. 通常，您将需要查看创建 struct 的方法，而不是 struct 本身
/// Iterator
/// 该模块的核心是 Iterator trait. Iterator 的核心如下:
/// ```
/// trait Iterator {
///     type Item;
///
///     fn next(&mut self) -> Option<Self::Item>;
/// }
/// ```
/// 迭代器有一个方法 next, 当调用它时, 返回一个 Option<Item>. 只要有元素 next 就会返回 Some(Item),
/// 一旦它们全部消费完, 将返回 None 表示迭代完成. 各个迭代器可能选择恢复迭代, 因此再次调用 next
/// 可能会或可能不会 开始再次返回 Some(Item) (例如，请参见 TryIter).
/// 迭代器也是可组合的, 通常将它们链接在一起以进行更复杂的处理形式.
///
/// # 三种迭代方式:
///   - iter(), 迭代在 &T 上
///   - iter_mut(), 迭代在 &mut T 上
///   - into_iter(), 迭代在 T 上.
/// 在适当的情况下，标准库中的各种东西都可以实现这三个中的一个或多个.
///
/// # 实现迭代器:
/// 创建自己的迭代器涉及两个步骤:
///   - 创建一个 struct 来保存迭代器的状态
///   - 为该 struct 实现 Iterator.
/// 这就是为什么此模块中有这么多 struct 的原因, 每个迭代器和迭代适配器都有一个.
///
/// # for 循环 和 IntoIterator
/// rust 的 for 循环语法实际上是迭代器的语法糖
/// ```
/// let values = vec![1, 2, 3, 4, 5];
/// for x in values {
///     println!("{}", x);
/// }
/// ```
/// 标准库中有一个 trait 用于将某些内容转换为迭代器: IntoIterator。
/// 这个 trait 具有一个 into_iter 方法，该方法可以将实现 IntoIterator 的类型转换为迭代器
/// 标准库包含一个有趣的 IntoIterator 实现,
/// ```
/// ipub trait IntoIterator {
///     type Item;
///     type IntoIter: Iterator
///     where
///         <Self::IntoIter as Iterator>::Item == Self::Item;
///     fn into_iter(self) -> Self::IntoIter;
/// }
///
/// impl<I: Iterator> IntoIterator for I
/// ```
/// 意味着所有 Iterator 都实现 IntoIterator. 仅仅返回自身, 这意味着两件事:
///   - 如果要编写 Iterator，则可以将其与 for 循环一起使用
///   - 如果要创建集合，则为其实现 IntoIterator 将使您的集合可以与 for 循环一起使用
///
/// # 通过引用进行迭代:
/// 由于 into_iter() 将 self 作为值，因此使用 for 循环遍历一个集合将消耗该集合。
/// 通常，您可能需要迭代一个集合而不消耗它. 许多集合提供了在引用上提供迭代器的方法，
/// 通常分别称为 iter() 和 iter_mut()：
/// 如果集合类型 C 提供 iter()，则它通常还为 &C 实现 IntoIterator，而该实现只是调用 iter()。
/// 同样，提供 iter_mut() 的集合 C 通常通过委派给 iter_mut() 来为 &mut C 实现 IntoIterator.
/// 尽管许多集合都提供 iter()，但并非所有集合都提供 iter_mut(). 例如: HashSet<T> 或 HashMap<K, V>
///
/// # Adapter 适配器
/// 接受一个 Iterator 并返回另一个 Iterator 的函数通常被称为迭代器适配器，因为它们是适配器模式的一种形式。
/// 常见的迭代器适配器包括 map，take 和 filter。
/// 如果迭代器适配器为 panics，则迭代器将处于未指定 (但内存安全) 状态。
/// 也不能保证此状态在 Rust 的各个版本中都保持不变，因此您应避免依赖 panicked 的迭代器返回的确切值。
///
/// # 惰性:
/// 迭代器 (和迭代器 适配器) 是懒惰的)。
/// 这意味着仅仅创建一个迭代器并不会做很多事情。除非您调用 next，否则什么都不会发生。
/// 当创建仅出于其副作用的迭代器时，这有时会引起混乱.
///
/// 无限:
/// 迭代器不必一定是有限的, 例如, 开放式范围是一个无限迭代器:
/// 通常使用 take 迭代器适配器将无限迭代器转换为有限迭代器：
/// ```
/// let numbers = 0..;
/// let five_numbers = numbers.take(5);
/// ```
///

#[cfg(test)]
mod tests {
    use std::iter;

    /// 迭代器的函数
    #[test]
    fn functions() {
        // Empty 创建一个不产生任何结果的迭代器。
        let mut nope = iter::empty::<i32>();
        assert_eq!(None, nope.next());

        // from_fn 创建一个新的迭代器, 每次迭代都调用提供的闭包, F: FnMut() -> Option<T>
        // 如重新实现计数器迭代器.
        let mut count = 0;
        let counter = std::iter::from_fn(move || {
            // 增加我们的数量。这就是为什么我们从零开始。
            count += 1;
            // 检查我们是否已经完成计数。
            if count < 6 {
                Some(count)
            } else {
                None
            }
        });
        assert_eq!(counter.collect::<Vec<_>>(), &[1, 2, 3, 4, 5]);
    }

    // # into_iterator
    /// ```
    /// pub trait IntoIterator {
    ///     type Item;
    ///     type IntoIter: Iterator
    ///         where
    ///          <Self::IntoIter as Iterator>::Item == Self::Item;
    ///
    ///     fn into_iter(self) -> Self::IntoIter;
    /// }
    /// ```
    /// 通过为类型实现 IntoIterator ，您可以定义如何将其转换为迭代器。这对于描述某种集合的类型很常见。
    /// 实现了 IntoIterator 的好处就是可以使用 rust for loop 的语法.
    #[test]
    fn into_iterator() {}

    ///```
    /// pub trait FromIterator<A> {
    ///     fn from_iter<T>(iter: T) -> Self
    //       where
    ///           T: IntoIterator<Item = A>;
    /// }
    /// ```
    /// 通过为类型实现 FromIterator ，您可以定义如何从迭代器创建它。这对于描述某种集合的类型很常见。
    /// 如果要从迭代器的内容创建一个集合，则首选  Iterator:: collect（）方法。但是，当您需要指定容器类型时
    /// FromIterator::from_iter() 很少被显式调用，而是通过 Iterator::collect() 方法使用。
    #[test]
    fn from_iterator() {}
}
