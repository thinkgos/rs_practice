use log::{debug, error, info, log_enabled, Level};

fn main() {
    error!("this is printed by default1");
    // 注意，env_logger 必须尽可能早的初始化
    env_logger::init();

    debug!("this is a debug {}", "message");
    error!("this is printed by default2");
    error!("this is printed by default3");
    if log_enabled!(Level::Info) {
        let x = 3 * 4; // expensive computation
        info!("the answer was: {}", x);
    }
}
