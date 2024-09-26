mod calculator;
mod sorter;
use std::io;

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    input.trim().to_string() 
}

fn main() {
    loop {
        println!("\n选择功能:");
        println!("1. 计算器");
        println!("2. 排序");
        println!("0. 退出");

        let choice = get_input();

        match choice.as_str() {
            "1" => calculator::calculate(),
            "2" => sorter::prompt_sorting(),
            "0" => {
                println!("退出程序");
                break;
            }
            _ => {
                println!("无效的选项，请重新输入");
            }
        }
    }
}
