use loan_payoff::{pay_loans_all_orderings, Loan};

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let loan1 = Loan::new("num1".to_owned(), 10000.00, 27.6 / 12.0 / 100.0, 23, 564.74);
    let loan2 = Loan::new("num2".to_owned(), 10000.00, 7.5 / 12.0 / 100.0, 48, 241.79);
    let loan3 = Loan::new(
        /*name:*/ "num3".to_owned(),
        /*initial_value:*/ 13000.00,
        /*rate:*/ 16.8 / 12.0 / 100.0,
        /*number_of_payments:*/ 48,
        /*payment_amount:*/ 373.77,
    );

    let extra_amount = 100.0;
    let loans = vec![&loan1, &loan2, &loan3];
    let optimal_payoff =
        pay_loans_all_orderings(&loans, extra_amount).expect("Failed to pay loans");

    println!(
        "Best ordering = {}, with savings ${}, is debt snowball {}, savings over debt snowball ${}",
        optimal_payoff
            .ordering
            .iter()
            .map(|&i| loans[i].name.as_ref())
            .collect::<Vec<_>>()
            .join(" -> "),
        optimal_payoff.savings,
        optimal_payoff.is_debt_snowball,
        optimal_payoff.savings_over_debt_snowball
    );
}
