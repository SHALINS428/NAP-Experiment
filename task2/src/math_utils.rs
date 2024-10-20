/// 计算n的阶乘
pub fn factorial(n: u32) -> u32 {
    (1..=n).product() // 计算从1到n的乘积
}

/// 计算斐波那契数列的第n项
pub fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
