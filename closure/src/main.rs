use anyhow::Result;
use closure::{Config, run};

use std::env;

/// 程序入口点
///
/// 从命令行参数解析配置，然后运行程序。
///
/// # 使用方式
/// ```bash
/// cargo run <query> <file_path>
/// ```
///
/// # 环境变量
/// * `IGNORE_CASE` - 如果设置此环境变量，搜索时将忽略大小写
///
/// # 返回
/// * `Result<()>` - 成功时返回 Ok(())，失败时返回错误
fn main() -> Result<()> {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 从命令行参数创建配置
    let config = Config::new(args.into_iter())?;

    // 打印配置信息
    eprintln!("query: {}", config.query);
    println!("file_path: {}", config.file_path);
    println!("ignore_case: {}", config.ignore_case);

    // 运行程序
    run(config)?;
    Ok(())
}
