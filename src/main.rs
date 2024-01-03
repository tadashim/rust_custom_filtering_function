struct CountDown {
    remaining: i32,
}

impl Iterator for CountDown {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining > 0 {
            let current = self.remaining;
            self.remaining -= 1;
            Some(current)
        } else {
            None
        }
    }
}

fn main() {
    let countdown = CountDown { remaining: 5 };

    for i in countdown {
        println!("Remaining: {}", i);
    }
}
