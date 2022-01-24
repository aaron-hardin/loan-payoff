use loan_payoff::{Loan};
use std::cmp;

const default_rounding_places: u8 = 4;

fn main() {
    let mut loan1 = Loan {
        name: "num1".to_owned(),
        present_value: 12000.00,
        rate: 0.006,
        number_of_payments: 48,
        payment_amount: 288.47
    };
    let mut loan2 = Loan {
        name: "num2".to_owned(),
        present_value: 10000.00,
        rate: 0.00625,
        number_of_payments: 48,
        payment_amount: 241.79
    };

    let mut loans = vec![loan1, loan2];

    let mut payment_amounts = Vec::new();
    let mut actual_costs = Vec::new();
    let mut expected_costs = Vec::new();
    for i in 0..loans.len() {
        let payment_amount = round_to_currency(loans[i].calculate_payment_amount());
        payment_amounts.push(payment_amount);
        if !approx_equal(payment_amount, loans[i].payment_amount, default_rounding_places) {
            println!("warning: calculated loan payment amount {} is not the same as given amount {}", payment_amount, loans[i].payment_amount);
        }

        actual_costs.push(0.0);
        expected_costs.push(round_to_currency(payment_amounts[i] * loans[i].number_of_payments as f64));
    }

    let mut count = 0;
    let extra_amount = 0.0;
    payment_amounts[0] = round_to_currency(payment_amounts[0] + extra_amount);
    payment_amounts[1] = round_to_currency(payment_amounts[1] + extra_amount);
    while (loans[0].present_value > 0.0 && !approx_equal(loans[0].present_value, 0.0, default_rounding_places))
            || (loans[1].present_value > 0.0 && !approx_equal(loans[1].present_value, 0.0, default_rounding_places)) {
        count += 1;

        if loans[0].present_value > 0.0 && !approx_equal(loans[0].present_value, 0.0, default_rounding_places) {
            let payment_amount_this_period = pay_loan(&mut loans[0], payment_amounts[0]);
            println!("paying {} .. count={}", payment_amount_this_period, count);
            actual_costs[0] = round_to_currency(actual_costs[0] + payment_amount_this_period);
        }

        if loans[1].present_value > 0.0 && !approx_equal(loans[1].present_value, 0.0, default_rounding_places) {
            let payment_amount_this_period = pay_loan(&mut loans[1], payment_amounts[1]);
            println!("paying {} .. count={}", payment_amount_this_period, count);
            actual_costs[1] = round_to_currency(actual_costs[1] + payment_amount_this_period);
        }
    }

    for i in 0..loans.len() {
        println!("EXPECTED=${}", expected_costs[i]);
        println!("ACTUAL=${}", actual_costs[i]);
        println!("By paying an extra ${}, you saved ${}", extra_amount, round_to_currency(expected_costs[i]-actual_costs[i]));
    }
}

// Returns the amount paid
fn pay_loan(loan: &mut Loan, payment_amount: f64) -> f64 {
    //let mut payment_amount = round_to_currency(loan.calculate_payment_amount());
    //let calculated_payment_amount = payment_amount;
    if approx_equal(payment_amount, 0.0, default_rounding_places) {
        println!("ERR: paying {}", payment_amount);
        return 0.0;
    }

    let interest = round_to_currency(loan.present_value * loan.rate);
    println!("interest={}", interest);
    loan.present_value += interest;
    // if (payment_amount+5.0) <= loan.present_value {
    //     payment_amount = round_to_currency(payment_amount + 5.0);
    // }
    let mut payment_amount_this_period = payment_amount;
    if payment_amount_this_period > loan.present_value {
        payment_amount_this_period = round_to_currency(loan.present_value);
    }
    loan.present_value = round_to_currency(loan.present_value);
    println!("BEFORE: {}", loan);
    //println!("paying {} .. count={}, amount_due={}", payment_amount, count, calculated_payment_amount);
    loan.present_value -= payment_amount_this_period;
    loan.present_value = round_to_currency(loan.present_value);
    loan.number_of_payments -= 1;

    println!("AFTER: {}", loan);

    payment_amount_this_period
}

fn approx_equal (a: f64, b: f64, decimal_places: u8) -> bool {
    let p = 10.0f64.powi(-(decimal_places as i32));
    (a-b).abs() < p
}

fn round_to_currency(a: f64) -> f64 {
    (a * 100.0).round() / 100.0
}