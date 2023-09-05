fn sum_u32(nums: &[u32]) -> Option<u32> {
    let mut sum = 0u32;
    for &num in nums {
        match sum.checked_add(num) {
            Some(result) => sum = result,
            None => return None, 
        }
    }
    Some(sum)
}

fn main() {
    let numbers = &[1, 2, 3, u32::MAX]; 
    let numbers2: &[u32; 4] = &[1,2,3,4];
    match sum_u32(numbers) {
        Some(result) => println!("求和结果: {}", result),
        None => println!("溢出发生，无法计算和"),
    }
    match sum_u32(numbers2) {
        Some(result) => println!("求和结果: {}", result),
        None => println!("溢出发生，无法计算和"),
    }
        
}
