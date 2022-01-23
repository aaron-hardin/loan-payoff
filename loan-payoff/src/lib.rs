#[derive(Clone, PartialEq)]
pub struct Loan {
    pub name: String,
    pub present_value: f64,
    pub rate: f64,
    pub number_of_payments: i64,
    pub payment_amount: f64,
}

impl Loan {
    pub fn calculate_payment_amount(&self) -> f64 {
        self.present_value * (self.rate * f64::powf(1.0+self.rate, self.number_of_payments as f64))
        /
        (f64::powf(1.0+self.rate, self.number_of_payments as f64) - 1.0)
    }
}