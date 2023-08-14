fn main() {
    println!("Hello, world!");
}

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        match self.list.pop() {
            Some(result) => {
                self.update_average();
                Some(result)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }
}
