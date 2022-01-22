// 包的crate::模块::子模块
use restaurant::restaurant::{hosting, serving};

fn main() {
    hosting::add_to_waitlist();
    serving::take_order();
}
