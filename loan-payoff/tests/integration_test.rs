use loan_payoff;

#[test]
fn basic_payoff() -> Result<(), loan_payoff::Error> {
	let loan1 = loan_payoff::Loan {
		name: "l1".to_owned(),
		initial_value: 10000.0,
		rate: 0.00625, // 7.5% annual
		number_of_payments: 48,
		payment_amount: 241.79,
	};
	let loan2 = loan_payoff::Loan {
		name: "l2".to_owned(),
		initial_value: 12000.0,
		rate: 0.02083, // 25% annual
		number_of_payments: 36,
		payment_amount: 477.12,
	};
	let loans = vec![&loan1, &loan2];
	let extra_amount = 100.0;
	let mut ordering = vec![0; loans.len()];
	for i in 0..ordering.len() {
		ordering[i] = i
	}
	let (is_debt_snowball, actual_costs_total, savings_total) =
		loan_payoff::pay_loans(&loans, extra_amount, &ordering)?;

	assert!(is_debt_snowball);
	assert!(actual_costs_total == 28244.9);
	assert!(savings_total == 537.34);
	Ok(())
}

#[test]
fn outperform_debt_snowball() -> Result<(), loan_payoff::Error> {
	let loan1 = loan_payoff::Loan {
		name: "l1".to_owned(),
		initial_value: 10000.0,
		rate: 0.00625, // 7.5% annual
		number_of_payments: 48,
		payment_amount: 241.79,
	};
	let loan2 = loan_payoff::Loan {
		name: "l2".to_owned(),
		initial_value: 12000.0,
		rate: 8.0 / 12.0 / 100.0, // 8% annual
		number_of_payments: 48,
		payment_amount: 292.96,
	};
	let loans = vec![&loan1, &loan2];
	let extra_amount = 100.0;
	let optimal_payoff = loan_payoff::pay_loans_all_orderings(&loans, extra_amount)?;

	assert!(optimal_payoff.is_debt_snowball == false);
	assert!(optimal_payoff.savings_over_debt_snowball == 32.15);
	assert!(optimal_payoff.savings == 667.26);
	assert!(optimal_payoff.ordering.get(0) == Some(&1));
	assert!(optimal_payoff.ordering.get(1) == Some(&0));
	Ok(())
}
