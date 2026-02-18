fn main() {
    let names = vec!["小白", "小黑", "小红"];
    let result = names.get(10);
    println!("找到了: {}", result.unwrap());
}
