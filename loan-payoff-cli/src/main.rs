use loan_payoff::{Loan};

const DEFAULT_ROUNDING_PLACES: u8 = 4;

fn main() {
    let loan1 = Loan::new(
        "num1".to_owned(),
        12000.00,
        0.006,
        48,
        288.47
    );
    let loan2 = Loan::new(
        "num2".to_owned(),
        11000.00,
        0.00625,
        48,
        265.97
    );
    let loan3 = Loan::new(
        /*name:*/ "num3".to_owned(),
        /*present_value:*/ 10000.00,
        /*rate:*/ 0.014,
        /*number_of_payments:*/ 48,
        /*payment_amount:*/ 287.52
    );

    pay_loans_all_orderings(vec!(&loan1, &loan2, &loan3));
}

fn pay_loans_all_orderings(loans: Vec<&Loan>) {
    // https://www.quickperm.org/
    let mut ordering = vec![0; loans.len()];
    for i in 0..ordering.len() { ordering[i] = i }

    // initial ordering
    let (is_debt_snowball, actual_costs_total, savings_total) = pay_loans(&loans, &ordering);

    let n = loans.len();
    let mut p = vec![0; n + 1];
    for i in 0..p.len() { p[i] = i }
    let mut i = 1;
    while i < n {
        p[i] -= 1;
        let j = if i % 2 == 0 { 0 } else { p[i] };
        ordering.swap(j, i);

        let (is_debt_snowball, actual_costs_total, savings_total) = pay_loans(&loans, &ordering);

        i = 1;
        while p[i] == 0 {
            p[i] = i;
            i += 1;
        } // end while (p[i] is equal to 0)
    } // end while (i < N)

    // TODO: print best ordering
    //println!("{}", ordering.iter().map(|&i| loans[i].name.clone()).collect::<Vec<String>>().join(" -> "));
}

fn pay_loans(loans: &Vec<&Loan>, ordering: &[usize]) -> (bool, f64, f64) {
    let mut remaining_amounts = vec![0.0; loans.len()];
    let mut actual_costs = vec![0.0; loans.len()];
    let mut expected_costs = vec![0.0; loans.len()];

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
    let mut extra_amount = 100.0;
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
        println!("{} - EXPECTED=${}", loans[i].name, expected_costs[i]);
        println!("{} - ACTUAL=${}", loans[i].name, actual_costs[i]);
        println!("{} - By paying an extra ${}, you saved ${}", loans[i].name, original_extra_amount, round_to_currency(expected_costs[i]-actual_costs[i]));
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

fn approx_equal (a: f64, b: f64, decimal_places: u8) -> bool {
    let p = 10.0f64.powi(-(decimal_places as i32));
    (a-b).abs() < p
}

fn round_to_currency(a: f64) -> f64 {
    (a * 100.0).round() / 100.0
}