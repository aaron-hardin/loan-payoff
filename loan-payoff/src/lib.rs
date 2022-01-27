use std::fmt;

#[derive(Clone, PartialEq)]
pub struct Loan {
    pub name: String,
    pub initial_value: f64,
    pub rate: f64,
    pub number_of_payments: i64,
    pub payment_amount: f64,
    //pub present_value: f64,
}

const DEFAULT_ROUNDING_PLACES: u8 = 4;

impl Loan {
    pub fn new(name: String, initial_value: f64, rate: f64, number_of_payments: i64, payment_amount: f64) -> Loan {
        Loan {
            name,
            initial_value,
            rate,
            number_of_payments,
            payment_amount,
            //present_value: initial_value
        }
    }

    pub fn calculate_payment_amount(&self) -> f64 {
        self.initial_value * (self.rate * f64::powf(1.0+self.rate, self.number_of_payments as f64))
        /
        (f64::powf(1.0+self.rate, self.number_of_payments as f64) - 1.0)
    }

    // pub fn reset(&mut self) -> f64 {
    //     self.present_value = self.initial_value;
    //     self.present_value
    // }

    // Returns the amount paid, remaining_amount
    pub fn pay_loan(&self, present_value: f64, payment_amount: f64) -> (f64, f64) {
        //let mut payment_amount = round_to_currency(loan.calculate_payment_amount());
        //let calculated_payment_amount = payment_amount;
        if approx_equal(payment_amount, 0.0, DEFAULT_ROUNDING_PLACES) {
            println!("ERR: paying {}", payment_amount);
            return (0.0, 0.0);
        }

        let mut present_value = present_value;
        let interest = round_to_currency(present_value * self.rate);
        //println!("interest={}", interest);
        present_value += interest;
        let mut payment_amount_this_period = payment_amount;
        if payment_amount_this_period > present_value {
            payment_amount_this_period = round_to_currency(present_value);
        }
        present_value = round_to_currency(present_value);
        //println!("BEFORE: {}", self);
        //println!("paying {} .. count={}, amount_due={}", payment_amount, count, calculated_payment_amount);
        present_value -= payment_amount_this_period;
        present_value = round_to_currency(present_value);
        //self.number_of_payments -= 1;

        //println!("AFTER: {}", self);

        (payment_amount_this_period, present_value)
    }
}

impl fmt::Display for Loan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, P={}, r={}, n={}, A={}", self.name, self.initial_value, self.rate, self.number_of_payments, self.payment_amount)
    }
}

pub fn approx_equal (a: f64, b: f64, decimal_places: u8) -> bool {
    let p = 10.0f64.powi(-(decimal_places as i32));
    (a-b).abs() < p
}

pub fn round_to_currency(a: f64) -> f64 {
    (a * 100.0).round() / 100.0
}