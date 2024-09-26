use std::io;

pub fn prompt_sorting() {
    println!("请输入要排序的数字（用空格分隔）：");
    let input = get_input();

    let mut nums: Vec<f64> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<f64>().ok())
        .collect();

    println!("选择排序方式：");
    println!("1. 快速排序");
    println!("2. 归并排序");
    
    let sort_choice = get_input();

    println!("选择排序顺序：");
    println!("1. 正序");
    println!("2. 逆序");

    let order_choice = get_input();

    match sort_choice.as_str() {
        "1" => quick_sort(&mut nums),
        "2" => merge_sort(&mut nums),
        _ => {
            println!("无效的排序选择");
            return;
        }
    }

    if order_choice == "2" {
        nums.reverse(); // 逆序排序
    }

    println!("排序结果：{:?}", nums);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    input.trim().to_string() // 返回去掉空格的字符串
}

fn quick_sort(arr: &mut [f64]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot_index = partition(arr);
    quick_sort(&mut arr[0..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..]);
}

fn partition(arr: &mut [f64]) -> usize {
    let pivot_index = arr.len() - 1;
    let pivot_value = arr[pivot_index];
    let mut i = 0;

    for j in 0..pivot_index {
        if arr[j] < pivot_value {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot_index);
    i
}

fn merge_sort(arr: &mut [f64]) {
    if arr.len() <= 1 {
        return;
    }
    let mid = arr.len() / 2;
    let mut left = arr[0..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    merge_sort(&mut left);
    merge_sort(&mut right);

    merge(arr, &left, &right);
}

fn merge(arr: &mut [f64], left: &[f64], right: &[f64]) {
    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}
