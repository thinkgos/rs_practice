#[cfg(test)]
mod tests {
    #[test]
    fn querying_the_variant() {
        // is_some, 当为 Some(x) 返回 true,  否则返回 false
        // is_some_and , 当为 Some(x) 时, 且 断言函数 为真时, 返回true, 否则返回 false
        // is_none, 当为 None 返回true, 否则返回 false
        let v_some: Option<u32> = Some(2);
        assert_eq!(v_some.is_some(), true);
        assert_eq!(v_some.is_some_and(|x| x > 1), true);
        assert_eq!(v_some.is_some_and(|x| x > 4), false);
        assert_eq!(v_some.is_none(), false);
        let v_none: Option<u32> = None;
        assert_eq!(v_none.is_some(), false);
        assert_eq!(v_none.is_some_and(|x| x > 1), false);
        assert_eq!(v_none.is_none(), true);
    }

    #[test]
    fn querying_as() {
        // as_ref 将 &Option<T> 转换为 Option<&T>
        // as_mut 将 &mut Option<T> 转换为 Option<&mut T>
        // as_deref 将 &Option<T> 转换为 Option<&T::Target>
        // as_deref_mut 将 &mut Option<T> 转换为 Option<&mut T::Target>
        // as_pin_ref 将 Pin<&Option<T>> 转换为 Option<Pin<&T>>
        // as_pin_mut 将 Pin<&mut Option<T>> 转换为 Option<Pin<&mut T>>
    }
    #[test]
    fn unwrap_to_value() {
        // 提取值:
        //   当它是 Some 变体时，这些方法提取 Option<T> 中包含的值。
        //   当它是 None 变体时, 则进行处理.
        // 方法: expect, unwrap, unwrap_or, unwrap_or_default, unwrap_or_else
        let v_value_default = 255;
        let v_value = 100;
        let v_some = Some(v_value);
        let v_none = None::<i32>;

        // expect panics 带有提供的自定义消息
        assert_eq!(v_value, v_some.expect("expect got some failed"));
        // unwrap panics None 则panic并带有默认信息
        assert_eq!(v_value, v_some.unwrap());
        // unwrap_or None 则返回提供的默认值
        assert_eq!(v_value, v_some.unwrap_or(v_value_default));
        assert_eq!(v_value_default, v_none.unwrap_or(v_value_default));
        // unwrap_or_default 返回 Some(T) 的 T 值, 如果为 None 则返回 T 的默认值(T 必须实现 Default trait)
        assert_eq!(v_value, v_some.unwrap_or_default());
        assert_eq!(0, v_none.unwrap_or_default());
        // unwrap_or_else 返回 Some(T) 的 T 值, 如果为 None 则返回 提供函数 的返回值
        assert_eq!(v_value, v_some.unwrap_or_else(|| 2 * 2));
        assert_eq!(4, v_none.unwrap_or_else(|| 2 * 2));
    }

    #[test]
    fn option_to_result() {
        // 转换包含的值:
        //   这些方法将 Option 转换为 Result：
        // 方法: ok_or, ok_or_else, transpose
        let v_value = 100;
        let v_some = Some(v_value);
        let v_none = None::<i32>;

        // ok_or 将 Some(v) 转换为 Ok(v)，使用提供的 err 值, 将 None 转换为 Err(err)
        assert_eq!(Ok(v_value), v_some.ok_or("failed"));
        assert_eq!(Err("failed"), v_none.ok_or("failed"));
        // ok_or_else 将 Some(v) 转换为 Ok(v)，使用提供的函数返回值做不err, 并将 None 转换为 Err(err) 的值
        assert_eq!(Ok(v_value), v_some.ok_or_else(|| "failed"));
        assert_eq!(Err("failed"), v_none.ok_or_else(|| "failed"));

        // transpose 将 Option<Result<T,E>> 转为 Result<Option<T>,E>
        // Some(Ok(x)) -> Ok(Some(x))
        // Some(Err(e)) -> Err(e)
        // None -> Ok(None)
        let v_some: Option<Result<i32, String>> = Some(Ok(10));
        assert_eq!(v_some.transpose(), Ok(Some(10)));
        let v_some_err: Option<Result<i32, String>> = Some(Err("failed".to_owned()));
        assert_eq!(v_some_err.transpose(), Err("failed".to_owned()));
        let v_none: Option<Result<i32, String>> = None;
        assert_eq!(v_none.transpose(), Ok(None));
    }

    #[test]
    fn option_to_option() {
        // 这些方法转换了 Some 变体：
        // 方法: filter, flatten
        let v_value = 100;
        let v_some = Some(v_value);
        let v_none = None::<i32>;
        // filter 使用提供的函数过滤 Some(v)
        // 当为 Some(v) 时, 并且 提供的闭包函数 返回true, 则返回 Some(v)
        // 其它情况, 返回 None
        assert_eq!(Some(100), v_some.filter(|v| v % 2 == 0));
        assert_eq!(None, v_some.filter(|v| (v + 1) % 2 == 0));
        assert_eq!(None, v_none.filter(|v| (v + 1) % 2 == 0));

        // flatten 从一个对象中剥离一层嵌套的 Option<Option<T>>
        // 注意: 只能剥离一层
        let v_some_some: Option<Option<u32>> = Some(Some(6));
        assert_eq!(Some(6), v_some_some.flatten());
        let v_some_some: Option<Option<u32>> = Some(None);
        assert_eq!(None, v_some_some.flatten());
        let v_some_some: Option<Option<u32>> = None;
        assert_eq!(None, v_some_some.flatten());
        // flatten 多层剥离
        let v_some_some_some: Option<Option<Option<u32>>> = Some(Some(Some(6)));
        assert_eq!(Some(Some(6)), v_some_some_some.flatten());
        assert_eq!(Some(6), v_some_some_some.flatten().flatten());
    }

    #[test]
    fn some_map_some() {
        // 这些方法将 Option<T> 转换为不同类型 Option<U>：
        // 方法: map
        let v_value = 100;
        let v_some = Some(v_value);
        let v_none = None::<i32>;
        // map 通过将提供的函数将 Option<T> 转换为 Option<U> 的值, None 值不变，
        assert_eq!(Some(200), v_some.map(|x| x * 2));
        assert_eq!(None, v_none.map(|x| x * 2));

        // 这些方法将 Option<T> 转换为不同类型 U 的值：
        // 方法: map_or, map_or_else
        // map_or 通过将 提供的函数 将 Option<T> 转换为 U = f(T) 的值, None 值返回提供 default 的值，
        assert_eq!("100", v_some.map_or("666".to_owned(), |x| x.to_string()));
        assert_eq!(200, v_some.map_or(666, |x| x * 2));
        assert_eq!("666", v_none.map_or("666".to_owned(), |x| x.to_string()));
        // map_or_else 通过将提供的函数将 Option<T> 转换为 U = f(T) 的值, None 值使用提供的 default 函数返回的值
        assert_eq!(
            "100",
            v_some.map_or_else(|| "666".to_owned(), |x| x.to_string())
        );
        assert_eq!(200, v_some.map_or_else(|| 666, |x| x * 2));
        assert_eq!(
            "666",
            v_none.map_or_else(|| "666".to_owned(), |x| x.to_string())
        );
    }

    #[test]
    fn some_use_zip() {
        // 这些方法结合了两个 Option 值的 Some 变体, 如果有一个为 None 则返回 None.
        // 方法: zip, zip_with
        let x_some = Some(1);
        let y_some = Some("hi");
        let z_none = None::<u8>;
        // zip 当 self 为 Some(s) 且 提供的为 Some(o),返回 Some((s, o))
        // 其它情况返回 None.
        assert_eq!(x_some.zip(y_some), Some((1, "hi")));
        assert_eq!(y_some.zip(x_some), Some(("hi", 1,)));
        assert_eq!(x_some.zip(z_none), None);
        assert_eq!(z_none.zip(x_some), None);

        // zip_with 当 self 为 Some(s) 且 提供的为 Some(o),返回 提供函数的 Some(f(s, o))
        // 其它情况返回 None.
        // #[derive(Debug, PartialEq)]
        // struct Point {
        //     x: f64,
        //     y: f64,
        // }

        // impl Point {
        //     fn new(x: f64, y: f64) -> Self {
        //         Self { x, y }
        //     }
        // }

        // let x = Some(17.5);
        // let y = Some(42.7);

        // assert_eq!(x.zip_with(y, Point::new), Some(Point { x: 17.5, y: 42.7 }));
        // assert_eq!(x.zip_with(None, Point::new), None);
    }

    #[test]
    fn boolean_operators_and() {
        // 这些方法将 Option 视为布尔值.
        // 其中 Some 的作用像 true，None 的作用像 false。
        // 这些方法有两类：
        //    一种 Option 作为输入
        //    一种 提供的函数作为输入(采用懒惰评估)

        // and  self     input          output
        //  -	None	(ignored)	    None
        //  - 	Some(x)	 Option<T>	    Option<T>
        // 简言之就是:
        //  self 为 None 则返回 None(即 Self)
        //  self 为 Some 则返回 input
        let x_none: Option<&str> = None;
        let y_some: Option<i32> = Some(2);
        let z_some: Option<&str> = Some("hello");
        assert_eq!(x_none.and(y_some), None);
        assert_eq!(y_some.and(x_none), None);
        assert_eq!(y_some.and(z_some), Some("hello"));
        // and_then     self          input          output
        //    -     	None	     (ignored)	      None
        //    -    	    Some(x)		   f(x)	          f(x)::<Option<T>>
        //  self 为 None 则返回 None(即 self)
        //  self 为 Some 则返回 f(v) 的结果, 也是 Option<T>
        assert_eq!(x_none.and_then(|v| Some(v)), None);
        assert_eq!(y_some.and_then(|v| Some(v * 2)), Some(4));
        assert_eq!(y_some.and_then(|_| None::<i32>), None);
    }

    #[test]
    fn boolean_operators_or() {
        // 这些方法将 Option 视为布尔值，
        // 其中 Some 的作用像 true，None 的作用像 false。
        // 这些方法有两类：
        //    一种 Option 作为输入
        //    一种 提供的函数作为输入(采用懒惰评估)

        // or   self      input         output
        //  - 	Some(x)	 (ignored)	    Some(x)
        //  -	None	  Option<y>   	Option<y>
        // 简言之就是:
        //    self 为 Some 则返回 Some(即 self)
        //    self 为 None 则返回 input
        let x_none: Option<i32> = None;
        let y_some: Option<i32> = Some(2);
        let z_some: Option<i32> = Some(200);
        assert_eq!(y_some.or(x_none), Some(2));
        assert_eq!(x_none.or(y_some), Some(2));
        assert_eq!(x_none.or(z_some), Some(200));
        // or_else      self          input          output
        //  - 	        Some(x)	      (ignored)	    Some(x)
        //  -	        None	       f()       	f()::Option<T>
        //  self 为 Some 则返回 Some(即 self)
        //  self 为 None 则返回 f() 的结果, 也是 Option<T>
        assert_eq!(y_some.or_else(|| Some(100)), y_some);
        assert_eq!(x_none.or_else(|| Some(100)), Some(100));
        assert_eq!(x_none.or_else(|| None), None);
    }
    #[test]

    fn boolean_operators_xor() {
        // 这些方法将 Option 视为布尔值，
        // 其中 Some 的作用像 true，None 的作用像 false。
        // 这些方法有两类：
        //    一种 Option 作为输入
        //    一种 提供的函数作为输入(采用懒惰评估)
        // 方法: xor

        // xor  self              input           output
        //  -  None/Some(x)	None/Some(y)       None
        //  -  None/Some(x)	Some(y)/None	  Some(y)/Some(x)
        // 简言之就是:
        //    self 和 input 同为 Some 或 None, 则返回 None,
        //    self 和 input 一个为 Some, 一个为 None, 则返回 Some,
        let x1_none: Option<i32> = None;
        let x2_none: Option<i32> = None;
        let y1_some: Option<i32> = Some(2);
        let y2_some: Option<i32> = Some(200);
        assert_eq!(x1_none.xor(x2_none), None);
        assert_eq!(y1_some.xor(y2_some), None);
        assert_eq!(x1_none.xor(y1_some), y1_some);
        assert_eq!(y2_some.xor(x2_none), y2_some);
    }
    #[test]
    fn comparison_operators() {
        // 如果 T 实现了 PartialOrd, 那么 Option<T> 也实现了 PartialOrd.
        // None 小于任意 Some, 而两个 Some 则比较他们 T 的值
        // 如果 T 实现了  Ord 则 Option<T> 也实现了.
        assert!(None < Some(0));
        assert!(Some(0) < Some(1));
    }

    #[test]
    fn iterating_over_option() {
        // Option 是可以迭代的, 需要迭器
        let yep = Some(42);
        let nope = None;

        // chain() already calls into_iter(), so we don't have to do so
        let nums: Vec<i32> = (0..4).chain(yep).chain(4..8).collect();
        assert_eq!(nums, [0, 1, 2, 3, 42, 4, 5, 6, 7]);
        let nums: Vec<i32> = (0..4).chain(nope).chain(4..8).collect();
        assert_eq!(nums, [0, 1, 2, 3, 4, 5, 6, 7]);

        fn make_iter(do_insert: bool) -> impl Iterator<Item = i32> {
            // Explicit returns to illustrate return types matching
            match do_insert {
                true => return (0..4).chain(Some(42)).chain(4..8),
                false => return (0..4).chain(None).chain(4..8),
            }
        }
        assert_eq!(
            make_iter(true).collect::<Vec<_>>(),
            [0, 1, 2, 3, 42, 4, 5, 6, 7]
        );
        assert_eq!(
            make_iter(false).collect::<Vec<_>>(),
            [0, 1, 2, 3, 4, 5, 6, 7]
        );
    }
    #[test]
    fn collecting_into_option() {
        // Option 实现了 FromIterator trait, 允许从 Option 迭化转为 Option 的 collected.
        // 如果有任一元素为 None, 则为 None
        let v_contain_none = [Some(2), Some(4), None, Some(8)];
        let res: Option<Vec<_>> = v_contain_none.into_iter().collect();
        assert_eq!(res, None);
        let v_all_some = [Some(2), Some(4), Some(8)];
        let res: Option<Vec<_>> = v_all_some.into_iter().collect();
        assert_eq!(res, Some(vec![2, 4, 8]));

        // Option 同样实现了 Product 和 Sum trait, 允许 迭化 Option 到 product 和 sum方法
        // 如果有任一元素为 None, 则为 None
        let v_contain_none = [None, Some(1), Some(2), Some(3)];
        let res: Option<i32> = v_contain_none.into_iter().sum();
        assert_eq!(res, None);
        let res: Option<i32> = v_contain_none.into_iter().product();
        assert_eq!(res, None);

        let v_all_some = [Some(1), Some(2), Some(21)];
        let res: Option<i32> = v_all_some.into_iter().sum();
        assert_eq!(res, Some(24));
        let res: Option<i32> = v_all_some.into_iter().product();
        assert_eq!(res, Some(42));
    }
    #[test]
    fn modifying_in_place() {
        // 这些方法返回对选项的包含值<t>的可变引用：
        // insert 丢弃旧值, 使用新值
        let mut y1: Option<i32> = Some(2);
        let y1 = y1.insert(100);
        assert_eq!(&100, y1);

        // get_or_insert 当为 None 时, 返回新值, 当为 Some 返回原值
        let mut y1: Option<i32> = None;
        let y1 = y1.get_or_insert(5);
        assert_eq!(&5, y1);
        let mut y1: Option<i32> = Some(2);
        let y1 = y1.get_or_insert(5);
        assert_eq!(&2, y1);
        //  get_or_insert_with 当为 None 时, 返回 函数的 计算值, 当为 Some 返回原值
        let mut y1: Option<i32> = None;
        let y1 = y1.get_or_insert_with(|| 5);
        assert_eq!(&5, y1);
        let mut y1: Option<i32> = Some(2);
        let y1 = y1.get_or_insert_with(|| 5);
        assert_eq!(&2, y1);

        //  get_or_insert_default 当为 None 时, 返回默认值, 当为 Some 返回原值
    }
    #[test]
    fn modifying_take_replace() {
        //  take 获取原值及所有权, 并且将原值设为 None
        let mut x = Some(2);
        let y = x.take();
        assert_eq!(x, None);
        assert_eq!(y, Some(2));

        let mut x: Option<u32> = None;
        let y = x.take();
        assert_eq!(x, None);
        assert_eq!(y, None);

        // replace 获取原值及所有权, 并且将原值设为 提供的值
        let mut x = Some(2);
        let old = x.replace(5);
        assert_eq!(x, Some(5));
        assert_eq!(old, Some(2));

        let mut x = None;
        let old = x.replace(3);
        assert_eq!(x, Some(3));
        assert_eq!(old, None);
    }
}
