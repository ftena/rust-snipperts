trait Widget {
    // Get the selected username out of this widget
    fn get(&self) -> (String, u8);
}

struct Form {
    username: String,
    age: u8,
}

impl Widget for Form {
    fn get(&self) -> (String, u8) {
        (self.username.clone(), self.age)
    }
}

fn main() {
    let form = Form {
        username: "rustacean".to_owned(),
        age: 28,
    };

    println!("{:?}", form.get());
}
