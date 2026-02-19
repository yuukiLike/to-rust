fn average(nums: &Vec<f64>) -> f64 {
    let len = nums.len() as f64;
    let mut sum: f64 = 0.0;
    for item in nums {
        sum += item
    }
    sum / len
}

fn main() {
    let nums = vec![10.0, 25.0, 30.0];
    println!("{:?}", average(&nums));
}
