#![feature(is_some_and)]
#![feature(result_flattening)]

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    #[test]
    fn querying_the_variant() {
        // is_ok, 当为 Ok(x) 返回 true,  否则返回 false
        // is_ok_and , 当为 Ok(x) 时, 且 断言函数 为真时, 返回true, 否则返回 false
        // is_err, 当为 Err(x) 返回true, 否则返回 false
        // is_err_and, 当为 Err(e) 时, 且 否则返回 为真时, 返回true, 否则返回 false
        let v_ok: Result<i32, i32> = Ok(6);
        assert_eq!(v_ok.is_ok(), true);
        assert_eq!(v_ok.is_err(), false);
        assert_eq!(v_ok.is_ok_and(|v| v % 2 == 0), true);
        assert_eq!(v_ok.is_ok_and(|v| v % 2 != 0), false);
        assert_eq!(v_ok.is_err_and(|e| e == 0), false);

        let v_err: Result<i32, i32> = Err(0);
        assert_eq!(v_err.is_ok(), false);
        assert_eq!(v_err.is_err(), true);
        assert_eq!(v_err.is_ok_and(|v| v % 2 == 0), false);
        assert_eq!(v_err.is_err_and(|e| e == 0), true);
    }

    #[test]
    fn querying_as() {
        // as_ref 将 &Result<T, E> 转换为 Result<&T, &E>
        // as_mut 将 &mut Result<T, E> 转换为 Result<&mut T, &mut E>
        // as_deref 将 &Result<T, E> 转换为 Result<&T::Target, &E>
        // as_deref_mut 将 &mut Result<T, E> 转换为 Result<&mut T::Target, &mut E>
    }
    #[test]
    fn unwrap_to_value() {
        // 提取值:
        //   当它是 Ok(T) 变体时，这些方法提取 Result<T, E> 中包含的T值。
        //   当它是 Err(E) 变体时, 则进行处理.
        // 方法: expect, unwrap, unwrap_or, unwrap_or_default, unwrap_or_else
        let v_default_value = 255;
        let v_value = 100;
        let v_ok: Result<i32, &str> = Ok(v_value);
        let v_err: Result<i32, &str> = Err("error");

        // expect panics 带有提供的自定义消息
        assert_eq!(v_value, v_ok.expect("expect got some failed"));
        // unwrap panics 带有泛型信息
        assert_eq!(v_value, v_ok.unwrap());
        // unwrap_or 如果这 Err(E) 则返回提供的默认值
        assert_eq!(v_value, v_ok.unwrap_or(v_default_value));
        assert_eq!(v_default_value, v_err.unwrap_or(v_default_value));
        // unwrap_or_default 返回 Ok(T) 的 T 值, 如果为 Err(E) 则返回 T 的默认值(T 必须实现 Default trait)
        assert_eq!(v_value, v_ok.unwrap_or_default());
        assert_eq!(0, v_err.unwrap_or_default());
        // unwrap_or_else 返回 Ok(T) 的 T 值, 如果为 Err(E) 则返回 提供函数 的返回值
        assert_eq!(v_value, v_ok.unwrap_or_else(|_e| 2 * 2));
        assert_eq!(4, v_err.unwrap_or_else(|_e| 2 * 2));
    }
    #[test]
    fn result_to_option() {
        // ok 将 Result<T, E> 转为 Option<T>. 其中 Err 转为 None

        let v_ok: Result<u32, &str> = Ok(2);
        let v_err: Result<u32, &str> = Err("Nothing here");
        assert_eq!(v_ok.ok(), Some(2));
        assert_eq!(v_err.ok(), None);
        // err 将 Result<T, E> 转为 Option<E>. 其中 Ok 转为 None
        assert_eq!(v_ok.err(), None);
        assert_eq!(v_err.err(), Some("Nothing here"));

        // transpose 将 Result<Option<T>, E> 转为 Option<Result<T, E>>
        // Ok(Some(x)) -> Some(Ok(x))
        // Ok(None)  -> None
        // Err(x) -> Some(Err(x))
        #[derive(Debug, Eq, PartialEq)]
        struct SomeErr;
        let v_ok: Result<Option<i32>, SomeErr> = Ok(Some(5));
        assert_eq!(v_ok.transpose(), Some(Ok(5)));
        let v_ok: Result<Option<i32>, SomeErr> = Ok(None);
        assert_eq!(v_ok.transpose(), None);
        let v_err: Result<Option<i32>, SomeErr> = Err(SomeErr);
        assert_eq!(v_err.transpose(), Some(Err(SomeErr)));
    }
    #[test]
    fn result_to_result() {
        // flatten 从一个对象中剥离一层嵌套的 Result<Result<T,E>,E>
        // 注意: 只支持剥离一层
        let x: Result<Result<&'static str, u32>, u32> = Ok(Ok("hello"));
        assert_eq!(Ok("hello"), x.flatten());
        let x: Result<Result<&'static str, u32>, u32> = Ok(Err(6));
        assert_eq!(Err(6), x.flatten());
        let x: Result<Result<&'static str, u32>, u32> = Err(6);
        assert_eq!(Err(6), x.flatten());
        // flatten 多层剥离
        let x: Result<Result<Result<&'static str, u32>, u32>, u32> = Ok(Ok(Ok("hello")));
        assert_eq!(Ok("hello"), x.flatten().flatten());
    }

    #[test]
    fn result_map_result() {
        // 这些方法将 Result<T, E> 转换为不同类型 Result<U, E> 的值：
        // map 通过将提供的函数将 Result<T, E> 转换为 Result<U, E> 的值, None 值不变，
        // map_err 通过将提供的函数将 Result<T, E> 转换为 Result<T, F> 的值, Ok(x) 值不变，

        // 这些方法将 Result<T, E> 转换为不同类型 U 的值：
        // map_or 通过将 提供的函数 将 Result<T, E> 转换为 U = f(T) 的值,   Err 值返回提供 default 的值，
        // map_or 通过将提供的函数将 Result<T, E> 转换为 U = f(T) 的值, Err 值使用提供的 default 函数返回的值
    }

    #[test]
    fn boolean_operators_and() {
        // 这些方法将 Result 视为布尔值.
        // 其中 Ok(T) 的作用像 true，Err(e) 的作用像 false。
        // 这些方法有两类：
        //    一种 Result 作为输入
        //    一种 提供的函数作为输入(采用懒惰评估)

        // and  self     input          output
        //  -	Err(e)	(ignored)	    Err(e)
        //  - 	OK(x)	 Result<T, E>	Result<T, E>
        // 简言之就是:
        //  self 为 Err(e) 则返回 Err(e)(即 self)
        //  self 为 OK(x) 则返回 input

        // and_then     self          input          output
        //    -     	Err(e)	     (ignored)	      Err(e)
        //    -    	    OK(x)		   f(x)	         f(x)::Result<T, E>
        //  self 为 Err(e) 则返回 None(即 self)
        //  self 为 Ok(x) 则返回 f(v) 的结果, 也是 Result<T, E>
    }
    #[test]
    fn boolean_operators_or() {
        // 这些方法将 Result 视为布尔值，
        // 其中 OK 的作用像 true，Err 的作用像 false。
        // 这些方法有两类：
        //    一种 Result 作为输入
        //    一种 提供的函数作为输入(采用懒惰评估)
        // 方法: or

        // or   self      input         output
        //  - 	Ok(x)	 (ignored)	    OK(x)
        //  -	Err(e)	 Result<T, E>   Result<T, E>
        // 简言之就是:
        //    self 为 Ok 则返回 Ok(即 self)
        //    self 为 Err 则返回 input

        // or_else      self          input          output
        //  - 	        OK(x)	      (ignored)	    Ok(x)
        //  -	        Err(e)	       f()       	f()::Result<T, E>
        //  self 为 Ok 则返回 Ok(即 self)
        //  self 为 Err 则返回 f() 的结果, 也是 Result<T, E>
    }
    #[test]
    fn comparison_operators() {
        // 如果 T, E 实现了 PartialOrd, 那么 Result<T, E> 也实现了 PartialOrd.
        // Ok 小于任意 Err, 而两个 Ok 或 Err 则比较他们 T 或 E 的值
        // 如果 T, E 实现了  Ord 则 Result<T, E> 也实现了.
        assert!(Ok(1) < Err(0));
        let x_ok: Result<i32, ()> = Ok(0);
        let y_ok = Ok(1);
        assert!(x_ok < y_ok);
        let x_err: Result<(), i32> = Err(0);
        let y_err = Err(1);
        assert!(x_err < y_err);
    }

    #[test]
    fn iterating_over_result() {
        // result 是可以迭代的, 需要迭器

        let mut results = vec![];
        let mut errs = vec![];
        let nums: Vec<_> = ["17", "not a number", "99", "-27", "768"]
            .into_iter()
            .map(u8::from_str)
            // Save clones of the raw `Result` values to inspect
            .inspect(|x| results.push(x.clone()))
            // Challenge: explain how this captures only the `Err` values
            .inspect(|x| errs.extend(x.clone().err()))
            .flatten()
            .collect();
        assert_eq!(errs.len(), 3);
        assert_eq!(nums, [17, 99]);
        println!("results {results:?}");
        println!("errs {errs:?}");
        println!("nums {nums:?}");
    }

    #[test]
    fn collecting_into_result() {
        // Result 实现了 FromIterator trait, 允许从 Result 迭化转为 Result 的 collected.
        // 如果有任一元素为 Err, 则为 Err
        let v_contain_err = [Ok(2), Ok(4), Err("err!"), Ok(8)];
        let res: Result<Vec<_>, &str> = v_contain_err.into_iter().collect();
        assert_eq!(res, Err("err!"));
        let v_all_ok = [Ok(2), Ok(4), Ok(8)];
        let res: Result<Vec<_>, &str> = v_all_ok.into_iter().collect();
        assert_eq!(res, Ok(vec![2, 4, 8]));

        // Result 同样实现了 Product 和 Sum trait, 允许 迭化 Result 到 product 和 sum方法
        // 如果有任一元素为 Err, 则为 Err
        let v_contain_err = [Err("error!"), Ok(1), Ok(2), Ok(3), Err("foo")];
        let res: Result<i32, &str> = v_contain_err.into_iter().sum();
        assert_eq!(res, Err("error!"));
        let v_all_ok = [Ok(1), Ok(2), Ok(21)];
        let res: Result<i32, &str> = v_all_ok.into_iter().product();
        assert_eq!(res, Ok(42));
    }
}
