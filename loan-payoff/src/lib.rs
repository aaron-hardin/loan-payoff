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

pub struct OptimalPayoff {
    pub ordering: Vec<usize>,
    pub savings: f64,
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

pub fn pay_loans_all_orderings(loans: &Vec<&Loan>, extra_amount: f64) -> OptimalPayoff {
    // https://www.quickperm.org/
    let mut ordering = vec![0; loans.len()];
    for i in 0..ordering.len() { ordering[i] = i }

    // initial ordering
    let mut best_savings = -1.0;
    let mut best_ordering = ordering.clone();
    let (_is_debt_snowball, _actual_costs_total, savings_total) = pay_loans(&loans, extra_amount, &ordering);
    if savings_total > best_savings {
        best_savings = savings_total;
    }

    let n = loans.len();
    let mut p = vec![0; n + 1];
    for i in 0..p.len() { p[i] = i }
    let mut i = 1;
    while i < n {
        p[i] -= 1;
        let j = if i % 2 == 0 { 0 } else { p[i] };
        ordering.swap(j, i);

        let (_is_debt_snowball, _actual_costs_total, savings_total) = pay_loans(&loans, extra_amount, &ordering);
        if savings_total > best_savings {
            best_savings = savings_total;
            best_ordering = ordering.clone();
        }

        i = 1;
        while p[i] == 0 {
            p[i] = i;
            i += 1;
        } // end while (p[i] is equal to 0)
    } // end while (i < N)

    OptimalPayoff {
        ordering: best_ordering,
        savings: best_savings,
    }
}

pub fn pay_loans(loans: &Vec<&Loan>, extra_amount: f64, ordering: &[usize]) -> (bool, f64, f64) {
    let mut remaining_amounts = vec![0.0; loans.len()];
    let mut actual_costs = vec![0.0; loans.len()];
    let mut expected_costs = vec![0.0; loans.len()];
    let mut extra_amount = extra_amount;

    let mut is_debt_snowball = true;
    let mut max_cost = 0.0;

    for &i in ordering.iter() {
        let payment_amount = round_to_currency(loans[i].calculate_payment_amount());
        if !approx_equal(payment_amount, loans[i].payment_amount, DEFAULT_ROUNDING_PLACES) {
            panic!("warning for loan '{}': calculated loan payment amount {} is not the same as given amount {}", loans[i].name, payment_amount, loans[i].payment_amount);
        }

        if max_cost > loans[i].initial_value {
            is_debt_snowball = false;
        } else {
            max_cost = loans[i].initial_value;
        }

        remaining_amounts[i] = loans[i].initial_value;
        expected_costs[i] = round_to_currency(loans[i].payment_amount * loans[i].number_of_payments as f64);
    }

    let mut count = 0;
    let original_extra_amount = extra_amount;
    while ordering.iter().any(|&i| remaining_amounts[i] > 0.0 && !approx_equal(remaining_amounts[i], 0.0, DEFAULT_ROUNDING_PLACES)) {
        count += 1;

        let mut extra_amount_this_period = extra_amount;
        for &ix in ordering.iter() {
            if remaining_amounts[ix] > 0.0 && !approx_equal(remaining_amounts[ix], 0.0, DEFAULT_ROUNDING_PLACES) {
                let amount_to_pay = loans[ix].payment_amount + extra_amount_this_period;

                //println!("BEFORE: {}, remaining={}", loans[ix], remaining_amounts[ix]);
                let (amount_paid_this_period, remaining_amount) = loans[ix].pay_loan(remaining_amounts[ix], amount_to_pay);
                //println!("AFTER: {}, remaining={}", loans[ix], remaining_amount);

                remaining_amounts[ix] = remaining_amount;
                extra_amount_this_period = amount_paid_this_period - amount_to_pay;
                //println!("paying {} .. count={}", amount_paid_this_period, count);
                actual_costs[ix] = round_to_currency(actual_costs[ix] + amount_paid_this_period);

                // If the loan goes to 0 after paying, add the monthly payment to extra_amount (after paying all loans)
                if approx_equal(remaining_amounts[ix], 0.0, DEFAULT_ROUNDING_PLACES) {
                    // Note: we can update extra_amount directly because it is not used until next period
                    extra_amount = round_to_currency(extra_amount + loans[ix].payment_amount);
                }
            }
        }
    }

    let mut expected_costs_total = 0.0;
    let mut actual_costs_total = 0.0;
    let mut savings_total = 0.0;
    for &i in ordering.iter() {
        expected_costs_total += expected_costs[i];
        actual_costs_total += actual_costs[i];
        savings_total += expected_costs[i]-actual_costs[i];
        // println!("{} - EXPECTED=${}", loans[i].name, expected_costs[i]);
        // println!("{} - ACTUAL=${}", loans[i].name, actual_costs[i]);
        // println!("{} - By paying an extra ${}, you saved ${}", loans[i].name, original_extra_amount, round_to_currency(expected_costs[i]-actual_costs[i]));
    }
    println!("{}", ordering.iter().map(|&i| loans[i].name.clone()).collect::<Vec<String>>().join(" -> "));

    actual_costs_total = round_to_currency(actual_costs_total);
    savings_total = round_to_currency(savings_total);
    println!("EXPECTED=${}", round_to_currency(expected_costs_total));
    println!("ACTUAL=${}", actual_costs_total);
    println!("By paying an extra ${}, you saved ${}", original_extra_amount, savings_total);
    println!("Total periods={}", count);

    (is_debt_snowball, actual_costs_total, savings_total)
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