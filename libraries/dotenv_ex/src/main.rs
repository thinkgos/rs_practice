use dotenv::dotenv;
use std::env;
//
fn main() {
    // 加载 .env 文件, 然后就可以使用了.
    // 注意 .env 的键值对都采用 大写的
    // 从 .env 文件中加载
    dotenv().ok();

    let log_level = env::var("LOG_LEVEL");
    let log_level = log_level.unwrap_or_else(|_| "DEBUG".to_string());
    println!("log level: {}", log_level);

    let port = env::var("PORT");
    let port = port.unwrap_or_else(|_| "8080".to_string());
    println!("PORT: {}", port);

    // cargo run
    // 从 .env 文件中获取
    // export PORT=9090  && cargo run
    // 从环境变量中获取, 如果没有, 从 .env 文件中获取, 否则使用默认值
    let port = env::var("PORT");
    let port = port.unwrap_or_else(|_| "8080".to_string());
    println!("PORT: {}", port);
}
