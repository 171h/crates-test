
use std::env;
pub fn test_path_join() -> Result<(), Box<dyn std::error::Error>> {
    // 获取当前工作目录
    let current_dir = env::current_dir()?;

    // 创建一个新目录的字符串表示
    let new_directory = "new/path";

    // 将新目录与当前目录拼接
    let joined_path = current_dir.join(new_directory);

    // 转换为字符串（若需要）
    let joined_path_string: String = joined_path.to_str().unwrap().to_string();

    println!("Joined path: {}", joined_path_string);

    Ok(())
}
