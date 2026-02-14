struct Student {
    name: String,
    age: i32,
}

fn main() {
    let s = Student {
        name: String::from("小白"),
        age: 20,
    };

    let name = &s.name;

    let age:i32 = s.age;
    println!("{}", name);
    println!("{}", age);
    println!("{}", s.age);
    println!("{}", s.name);
}
