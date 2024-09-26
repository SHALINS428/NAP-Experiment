use std::io;

pub fn sort_numbers(nums: Vec<i32>, algorithm: &str) -> Vec<i32> {
    match algorithm {
        "1" => quick_sort(nums),
        "2" => merge_sort(nums),
        _ => nums, // 默认返回原始数组
    }
}

fn quick_sort(mut nums: Vec<i32>) -> Vec<i32> {
    if nums.len() <= 1 {
        return nums;
    }
    let pivot = nums[0];
    let less: Vec<i32> = nums.iter().cloned().filter(|&x| x < pivot).collect();
    let greater: Vec<i32> = nums.iter().cloned().filter(|&x| x > pivot).collect();
    [quick_sort(less), vec![pivot], quick_sort(greater)].concat()
}

fn merge_sort(nums: Vec<i32>) -> Vec<i32> {
    if nums.len() <= 1 {
        return nums;
    }
    let mid = nums.len() / 2;
    let left = merge_sort(nums[0..mid].to_vec());
    let right = merge_sort(nums[mid..].to_vec());
    merge(left, right)
}

fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let (mut i, mut j) = (0, 0);
    
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }
    
    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);
    
    result
}

pub fn prompt_sorting() {
    let mut nums = String::new();
    println!("请输入要排序的数字，以空格分隔：");
    io::stdin().read_line(&mut nums).unwrap();
    let nums: Vec<i32> = nums
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    println!("\n选择排序算法:");
    println!("1. 快速排序");
    println!("2. 归并排序");
    
    let mut algorithm = String::new();
    io::stdin().read_line(&mut algorithm).unwrap();
    let algorithm = algorithm.trim();

    let sorted_nums = sort_numbers(nums, algorithm);
    println!("排序后的数字: {:?}", sorted_nums);
}
