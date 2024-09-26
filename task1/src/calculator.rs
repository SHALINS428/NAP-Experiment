use std::io;

pub fn calculate() {
    loop {
        println!("\n选择功能:");
        println!("1. 加法");
        println!("2. 减法");
        println!("3. 乘法");
        println!("4. 除法");
        println!("0. 返回主菜单");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        if choice == "0" {
            break; // 返回主菜单
        }

        let mut num1 = String::new();
        println!("请输入第一个数字：");
        io::stdin().read_line(&mut num1).unwrap();
        let num1: f64 = num1.trim().parse().unwrap();

        if choice != "5" && choice != "6" {
            let mut num2 = String::new();
            println!("请输入第二个数字：");
            io::stdin().read_line(&mut num2).unwrap();
            let num2: f64 = num2.trim().parse().unwrap();

            match choice {
                "1" => println!("{} + {} = {}", num1, num2, add(num1, num2)),
                "2" => println!("{} - {} = {}", num1, num2, subtract(num1, num2)),
                "3" => println!("{} * {} = {}", num1, num2, multiply(num1, num2)),
                "4" => match divide(num1, num2) {
                    Ok(result) => println!("{} / {} = {}", num1, num2, result),
                    Err(e) => println!("{}", e),
                },
                _ => {}
            }
        } else {
            // 处理平方和开方
            match choice {
                "5" => println!("{} 的平方 = {}", num1, square(num1)),
                "6" => match square_root(num1) {
                    Ok(result) => println!("{} 的平方根 = {}", num1, result),
                    Err(e) => println!("{}", e),
                },
                _ => {}
            }
        }
    }
}

// 加法、减法、乘法、除法、平方和开方函数...

pub fn add(x: f64, y: f64) -> f64 {
    x + y
}

pub fn subtract(x: f64, y: f64) -> f64 {
    x - y
}

pub fn multiply(x: f64, y: f64) -> f64 {
    x * y
}

pub fn divide(x: f64, y: f64) -> Result<f64, String> {
    if y != 0.0 {
        Ok(x / y)
    } else {
        Err(String::from("错误：不能除以零"))
    }
}

pub fn square(x: f64) -> f64 {
    x * x
}

pub fn square_root(x: f64) -> Result<f64, String> {
    if x >= 0.0 {
        Ok(x.sqrt())
    } else {
        Err(String::from("错误：负数没有实数平方根"))
    }
}
