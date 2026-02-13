struct Student {
    name: String,
    age: i32,
    score: f64,
}

impl Student {
    fn introduce(&self) {
        println!("我叫{}，{}岁", self.name, self.age);
    }

    fn set_score(&mut self, new_score: f64) {
        self.score = new_score;
    }
}

fn main() {
    let mut s = Student {
        name: String::from("小白"),
        age: 18,
        score: 95.5,
    };

    s.set_score(100.0);
    s.introduce();
}
