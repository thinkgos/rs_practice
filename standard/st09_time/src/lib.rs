#[cfg(test)]
mod tests {
    use std::{
        thread,
        time::{self, Duration, Instant, SystemTime},
    };

    #[test]
    fn duration() {
        // Duration 代表时间跨度, 通常用于系统超时.
        // Duration 由内部秒和纳秒组成.
        let t = Duration::new(2, 0);
        assert_eq!(2, t.as_secs());
    }

    #[test]
    fn instant() {
        // Instant 单调非递减时钟的度量, 不透明且仅对 Duration 有用.
        // 它仅允许测量两个瞬间之间的持续时间. 无时间锚
        let now = Instant::now();
        thread::sleep(Duration::new(2, 0));
        assert_eq!(2, now.elapsed().as_secs())
    }
    #[test]
    fn sys_time_time() {
        // 系统时钟的度量，对于与文件系统或其他进程之类的外部实体进行通信很有用。
        // 与 Instant 类型不同，这类的测量 不是单调的, 且有时间锚
        // UNIX_EPOCH: 时间锚, 从 1970-01-01 00:00:00 UTC

        let t = SystemTime::now().duration_since(time::UNIX_EPOCH);
        match t {
            Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }
    }
}
