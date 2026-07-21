pub struct Calculator {
    pub value: f64,
}

impl Calculator {
    pub fn new(initial: f64) -> Calculator {
        Calculator { value: initial }
    }

    pub fn add(a: f64, b: f64) -> f64 {
        a + b
    }

    pub fn multiply(a: f64, b: f64) -> f64 {
        a * b
    }

    // This should be skipped because it requires an instance.
    pub fn get_value(&self) -> f64 {
        self.value
    }
}

pub fn standalone(x: f64) -> f64 {
    x * 2.0
}
