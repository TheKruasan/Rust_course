pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
//mplementations of the public methods add, remove, and average on AveragedCollection
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {//public method
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {//public method
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {//public method
        self.average
    }

    fn update_average(&mut self) {//private method
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}


fn main(){

}