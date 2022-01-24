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
    let mut extra_amount = 100.0;
    let original_extra_amount = extra_amount;
    while loans.iter().any(|loan| loan.present_value > 0.0 && !approx_equal(loan.present_value, 0.0, default_rounding_places)) {
        count += 1;

        let mut extra_amount_this_period = extra_amount;
        for ix in 0..loans.len() {
            if loans[ix].present_value > 0.0 && !approx_equal(loans[ix].present_value, 0.0, default_rounding_places) {
                let amount_to_pay = payment_amounts[ix] + extra_amount_this_period;
                let amount_paid_this_period = pay_loan(&mut loans[ix], amount_to_pay);
                extra_amount_this_period = amount_paid_this_period - amount_to_pay;
                println!("paying {} .. count={}", amount_paid_this_period, count);
                actual_costs[ix] = round_to_currency(actual_costs[ix] + amount_paid_this_period);

                // If the loan goes to 0 after paying, add the monthly payment to extra_amount (after paying all loans)
                if approx_equal(loans[ix].present_value, 0.0, default_rounding_places) {
                    // Note: we can update extra_amount directly because it is not used until next period
                    extra_amount = round_to_currency(extra_amount + payment_amounts[ix]);
                }
            }
        }
    }

    let mut expected_costs_total = 0.0;
    let mut actual_costs_total = 0.0;
    let mut savings_total = 0.0;
    for i in 0..loans.len() {
        expected_costs_total += expected_costs[i];
        actual_costs_total += actual_costs[i];
        savings_total += expected_costs[i]-actual_costs[i];
        println!("{} - EXPECTED=${}", loans[i].name, expected_costs[i]);
        println!("{} - ACTUAL=${}", loans[i].name, actual_costs[i]);
        println!("{} - By paying an extra ${}, you saved ${}", loans[i].name, original_extra_amount, round_to_currency(expected_costs[i]-actual_costs[i]));
    }

    println!("EXPECTED=${}", round_to_currency(expected_costs_total));
    println!("ACTUAL=${}", round_to_currency(actual_costs_total));
    println!("By paying an extra ${}, you saved ${}", original_extra_amount, round_to_currency(savings_total));
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