use loan_payoff::{Loan, pay_loans_all_orderings};

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

    let extra_amount = 100.0;
    let loans = vec!(&loan1, &loan2, &loan3);
    let (best_ordering, best_savings) = pay_loans_all_orderings(&loans, extra_amount);

    println!(
        "Best ordering = {}, with savings ${}",
        best_ordering.iter().map(|&i| loans[i].name.clone()).collect::<Vec<String>>().join(" -> "),
        best_savings
    );
}
