use std::io;

pub fn calculate() {
    loop {
        println!("\n选择操作:");
        println!("1. 加法");
        println!("2. 减法");
        println!("3. 乘法");
        println!("4. 除法");
        println!("0. 返回主菜单");

        let operation = get_input();

        match operation.as_str() {
            "1" => perform_operation("+"),
            "2" => perform_operation("-"),
            "3" => perform_operation("*"),
            "4" => perform_operation("/"),
            "0" => break,
            _ => println!("无效的选项，请重新输入"),
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    input.trim().to_string() // 返回去掉空格的字符串
}

fn perform_operation(op: &str) {
    println!("请输入第一个数字:");
    let num1 = get_input();
    let num1: f64 = match num1.parse() {
        Ok(n) => n,
        Err(_) => {
            println!("请输入一个有效的数字");
            return;
        }
    };

    println!("请输入第二个数字:");
    let num2 = get_input();
    let num2: f64 = match num2.parse() {
        Ok(n) => n,
        Err(_) => {
            println!("请输入一个有效的数字");
            return;
        }
    };

    match op {
        "+" => println!("结果: {}", num1 + num2),
        "-" => println!("结果: {}", num1 - num2),
        "*" => println!("结果: {}", num1 * num2),
        "/" => {
            if num2 != 0.0 {
                println!("结果: {}", num1 / num2);
            } else {
                println!("错误: 除数不能为零");
            }
        }
        _ => println!("无效的操作"),
    }
}
