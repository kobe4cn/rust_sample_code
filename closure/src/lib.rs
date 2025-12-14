#[derive(Debug, PartialEq, Eq, Clone)]
#[allow(unused)]
enum ShirtColor {
    Red,
    Blue,
}
#[allow(unused)]
struct Inventory {
    shirts: Vec<ShirtColor>,
}

#[allow(unused)]
impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

use rand::Rng;
use std::{cmp::Ordering, env};

use anyhow::Result;

/// 配置结构体
///
/// 用于存储程序的配置参数，包括查询字符串、文件路径和是否忽略大小写的标志。
pub struct Config {
    /// 要搜索的查询字符串
    pub query: String,
    /// 要搜索的文件路径
    pub file_path: String,
    /// 是否忽略大小写进行搜索
    pub ignore_case: bool,
}

impl Config {
    /// 从命令行参数创建配置
    ///
    /// # 参数
    /// * `args` - 命令行参数的迭代器
    ///
    /// # 返回
    /// * `Result<Config>` - 成功时返回配置对象，失败时返回错误
    ///
    /// # 错误
    /// 如果缺少查询字符串或文件路径参数，将返回错误。
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config> {
        // 跳过第一个参数（程序名称）
        args.next();

        // 获取查询字符串（第二个参数）
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err(anyhow::anyhow!("Didn't get a query string")),
        };

        // 获取文件路径（第三个参数）
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err(anyhow::anyhow!("Didn't get a file path")),
        };

        // 检查环境变量 IGNORE_CASE 是否设置，用于决定是否忽略大小写
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        println!("ignore_case: {}", ignore_case);

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
use std::fs;

/// 运行程序的主函数
///
/// 根据配置读取文件内容并打印出来。
///
/// # 参数
/// * `config` - 包含查询字符串、文件路径等配置信息的 Config 结构体
///
/// # 返回
/// * `Result<()>` - 成功时返回 Ok(())，失败时返回错误
pub fn run(config: Config) -> Result<()> {
    // 读取文件内容
    let contents = fs::read_to_string(config.file_path)?;
    println!("contents: {}", contents);
    Ok(())
}

/// 在内容中搜索包含查询字符串的行（区分大小写）
///
/// # 参数
/// * `query` - 要搜索的查询字符串
/// * `contents` - 要搜索的内容文本
///
/// # 返回
/// * `Vec<&'a str>` - 包含查询字符串的所有行的向量
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines() // 将内容按行分割
        .filter(|line| line.contains(query)) // 过滤出包含查询字符串的行
        .collect() // 收集结果到向量中
}

/// 在内容中搜索包含查询字符串的行（不区分大小写）
///
/// # 参数
/// * `query` - 要搜索的查询字符串
/// * `contents` - 要搜索的内容文本
///
/// # 返回
/// * `Vec<&'a str>` - 包含查询字符串的所有行的向量（不区分大小写）
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines() // 将内容按行分割
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase())) // 转换为小写后进行比较
        .collect() // 收集结果到向量中
}
/// 猜数字游戏函数
///
/// 生成一个1到100之间的随机数，让用户猜测。
/// 用户可以通过输入 'quit' 来退出游戏。
#[allow(unused)]
fn guess() {
    // 创建随机数生成器
    let mut rng = rand::rng();
    // 生成1到100之间的随机数作为秘密数字
    let secert_number: u32 = rng.random_range(1..=100);

    // 游戏主循环
    loop {
        println!("Your guess number: (type 'quit' to exit)");
        let mut guess = String::new();

        // 从标准输入读取用户的猜测
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 检查用户是否想退出游戏
        if guess.trim() == "quit" {
            println!("You quit the game!");
            break;
        }

        // 将输入的字符串解析为数字
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // 如果解析失败，跳过本次循环
        };

        // 比较猜测的数字与秘密数字
        match guess.cmp(&secert_number) {
            Ordering::Less => println!("Too small!"),  // 猜测的数字太小
            Ordering::Greater => println!("Too big!"), // 猜测的数字太大
            Ordering::Equal => {
                println!("You win!"); // 猜对了，游戏结束
                break;
            }
        }
    }
}
#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;

    #[test]
    fn test_giveaway() {
        let store = Inventory {
            shirts: vec![
                ShirtColor::Red,
                ShirtColor::Blue,
                ShirtColor::Red,
                ShirtColor::Red,
            ],
        };
        let giveaway = store.giveaway(None);
        println!("giveaway none preference: {:?}", giveaway);
        assert_eq!(giveaway, ShirtColor::Red);
    }
    #[test]
    fn test_giveaway_with_preference() {
        let store = Inventory {
            shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red],
        };
        let giveaway = store.giveaway(Some(ShirtColor::Red));
        println!("giveaway: {:?}", giveaway);
        assert_eq!(giveaway, ShirtColor::Red);
    }
    /// 测试区分大小写的搜索功能
    ///
    /// 验证 search 函数能够正确找到包含查询字符串的行。
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    /// 测试不区分大小写的搜索功能
    ///
    /// 验证 search_case_insensitive 函数能够忽略大小写找到匹配的行。
    #[test]
    fn one_result_case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Duct tape.";
        assert_eq!(vec!["Rust:"], search_case_insensitive(query, contents));
    }
}
