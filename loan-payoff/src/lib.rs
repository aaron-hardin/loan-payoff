use log;
use serde::Deserialize;
use std::fmt;

#[derive(Clone, PartialEq, Deserialize, Default)]
pub struct Loan {
	pub name: String,
	pub initial_value: f64,
	pub rate: f64,
	pub number_of_payments: i64,
	pub payment_amount: f64,
}

pub struct OptimalPayoff {
	pub ordering: Vec<usize>,
	pub savings: f64,
	pub is_debt_snowball: bool,
	pub savings_over_debt_snowball: f64,
}

const DEFAULT_ROUNDING_PLACES: u8 = 4;

impl Loan {
	pub fn new(
		name: String,
		initial_value: f64,
		rate: f64,
		number_of_payments: i64,
		payment_amount: f64,
	) -> Loan {
		Loan {
			name,
			initial_value,
			rate,
			number_of_payments,
			payment_amount,
		}
	}

	pub fn calculate_payment_amount(&self) -> f64 {
		self.initial_value
			* (self.rate * f64::powf(1.0 + self.rate, self.number_of_payments as f64))
			/ (f64::powf(1.0 + self.rate, self.number_of_payments as f64) - 1.0)
	}

	// Returns the amount paid, remaining_amount
	pub fn pay_loan(&self, present_value: f64, payment_amount: f64) -> (f64, f64) {
		if approx_equal(payment_amount, 0.0, DEFAULT_ROUNDING_PLACES) {
			println!("ERR: paying {}", payment_amount);
			return (0.0, 0.0);
		}

		let mut present_value = present_value;
		let interest = round_to_currency(present_value * self.rate);
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

pub fn pay_loans_all_orderings(
	loans: &Vec<&Loan>,
	extra_amount: f64,
) -> Result<OptimalPayoff, Error> {
	// https://www.quickperm.org/
	let mut ordering = vec![0; loans.len()];
	for i in 0..ordering.len() {
		ordering[i] = i
	}

	// initial ordering
	let mut best_savings = -1.0;
	let mut best_ordering = Vec::new();
	let mut best_debt_snowball_savings = -1.0; // Note: there can be multiple debt snowball (2 loans with same amount)
	let mut is_snowball_best = false;
	match pay_loans(&loans, extra_amount, &ordering) {
		Ok((is_debt_snowball, _actual_costs_total, savings_total)) => {
			if savings_total > best_savings {
				best_savings = savings_total;
				best_ordering = ordering.clone();
				is_snowball_best = is_debt_snowball;
			}
			if is_debt_snowball && savings_total > best_debt_snowball_savings {
				best_debt_snowball_savings = savings_total;
			}
		}
		Err(Error::LoanGoesToInf) => { /* noop: if no loan orderings converge then we return this error below */
		}
		Err(e) => return Err(e),
	}

	let n = loans.len();
	let mut p = vec![0; n + 1];
	for i in 0..p.len() {
		p[i] = i
	}
	let mut i = 1;
	while i < n {
		p[i] -= 1;
		let j = if i % 2 == 0 { 0 } else { p[i] };
		ordering.swap(j, i);

		match pay_loans(&loans, extra_amount, &ordering) {
			Ok((is_debt_snowball, _actual_costs_total, savings_total)) => {
				if savings_total > best_savings {
					best_savings = savings_total;
					best_ordering = ordering.clone();
					is_snowball_best = is_debt_snowball;
				}
				if is_debt_snowball && savings_total > best_debt_snowball_savings {
					best_debt_snowball_savings = savings_total;
				}
			}
			Err(Error::LoanGoesToInf) => { /* noop: if no loan orderings converge then we return this error below */
			}
			Err(e) => return Err(e),
		}

		i = 1;
		while p[i] == 0 {
			p[i] = i;
			i += 1;
		} // end while (p[i] is equal to 0)
	} // end while (i < N)

	if best_savings < 0.0 {
		return Err(Error::LoanGoesToInf);
	}

	Ok(OptimalPayoff {
		ordering: best_ordering,
		savings: best_savings,
		is_debt_snowball: is_snowball_best,
		savings_over_debt_snowball: round_to_currency(best_savings - best_debt_snowball_savings),
	})
}

pub fn pay_loans(
	loans: &Vec<&Loan>,
	extra_amount: f64,
	ordering: &[usize],
) -> Result<(bool, f64, f64), Error> {
	log::debug!("Pay loans {:?}", ordering);
	let mut remaining_amounts = vec![0.0; loans.len()];
	let mut actual_costs = vec![0.0; loans.len()];
	let mut expected_costs = vec![0.0; loans.len()];
	let mut extra_amount = extra_amount;

	let mut is_debt_snowball = true;
	let mut max_cost = 0.0;

	let mut max_number_payments = 0;

	for &i in ordering.iter() {
		let payment_amount = round_to_currency(loans[i].calculate_payment_amount());
		if !within_five_cents(payment_amount, loans[i].payment_amount) {
			log::error!("loan '{}': calculated loan payment amount {} is not within 5 cents of given amount {}", loans[i].name, payment_amount, loans[i].payment_amount);
			return Err(Error::InvalidLoan(i));
		}

		if max_cost > loans[i].initial_value {
			is_debt_snowball = false;
		} else {
			max_cost = loans[i].initial_value;
		}

		if loans[i].number_of_payments > max_number_payments {
			max_number_payments = loans[i].number_of_payments;
		}

		remaining_amounts[i] = loans[i].initial_value;
		expected_costs[i] =
			round_to_currency(loans[i].payment_amount * loans[i].number_of_payments as f64);
	}

	let mut count = 0;
	let original_extra_amount = extra_amount;
	while ordering.iter().any(|&i| {
		remaining_amounts[i] > 0.0
			&& !approx_equal(remaining_amounts[i], 0.0, DEFAULT_ROUNDING_PLACES)
	}) {
		count += 1;

		if count > max_number_payments {
			for &ix in ordering.iter() {
				log::error!(
					"Loan={}, Remaining Amount={}",
					loans[ix],
					remaining_amounts[ix]
				);
			}
			log::error!(
				"Went too long, should have finished in at most {} periods",
				max_number_payments
			);
			return Err(Error::LoanGoesToInf);
		}

		let mut extra_amount_this_period = extra_amount;
		for &ix in ordering.iter() {
			if remaining_amounts[ix] > 0.0
				&& !approx_equal(remaining_amounts[ix], 0.0, DEFAULT_ROUNDING_PLACES)
			{
				let amount_to_pay = loans[ix].payment_amount + extra_amount_this_period;

				log::trace!(
					"BEFORE {}: {}, remaining={}",
					count,
					loans[ix],
					remaining_amounts[ix]
				);
				let (amount_paid_this_period, remaining_amount) =
					loans[ix].pay_loan(remaining_amounts[ix], amount_to_pay);
				log::trace!(
					"AFTER {}: {}, remaining={}",
					count,
					loans[ix],
					remaining_amount
				);

				remaining_amounts[ix] = remaining_amount;
				extra_amount_this_period = amount_paid_this_period - amount_to_pay;
				log::trace!("paying {} .. count={}", amount_paid_this_period, count);
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
		savings_total += expected_costs[i] - actual_costs[i];
		// println!("{} - EXPECTED=${}", loans[i].name, expected_costs[i]);
		// println!("{} - ACTUAL=${}", loans[i].name, actual_costs[i]);
		// println!("{} - By paying an extra ${}, you saved ${}", loans[i].name, original_extra_amount, round_to_currency(expected_costs[i]-actual_costs[i]));
	}
	println!(
		"{}",
		ordering
			.iter()
			.map(|&i| loans[i].name.as_ref())
			.collect::<Vec<_>>()
			.join(" -> ")
	);

	actual_costs_total = round_to_currency(actual_costs_total);
	savings_total = round_to_currency(savings_total);
	println!("EXPECTED=${}", round_to_currency(expected_costs_total));
	println!("ACTUAL=${}", actual_costs_total);
	println!(
		"By paying an extra ${}, you saved ${}",
		original_extra_amount, savings_total
	);
	println!("Is debt snowball {}", is_debt_snowball);
	println!("Total periods={}", count);

	log::info!(
		"Pay loans with ordering {:?}, total amount {}, savings {}",
		ordering,
		actual_costs_total,
		savings_total
	);

	Ok((is_debt_snowball, actual_costs_total, savings_total))
}

impl fmt::Display for Loan {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{}, P={}, r={}, n={}, A={}",
			self.name, self.initial_value, self.rate, self.number_of_payments, self.payment_amount
		)
	}
}

fn within_five_cents(a: f64, b: f64) -> bool {
	(a - b).abs() <= 0.05
}

fn approx_equal(a: f64, b: f64, decimal_places: u8) -> bool {
	let factor = 10.0f64.powi(decimal_places as i32);
	let a = (a * factor).round();
	let b = (b * factor).round();
	a == b
}

pub fn round_to_currency(a: f64) -> f64 {
	(a * 100.0).round() / 100.0
}

pub fn round_to_decimals(a: f64, places: i32) -> f64 {
	(a * 10.0f64.powi(places)).round() / 10.0f64.powi(places)
}

#[derive(Debug)]
pub enum Error {
	LoanGoesToInf,
	OtherError(String),
	InvalidLoan(usize),
}

#[cfg(test)]
mod tests {
	use super::*;
	use test_case::test_case;

	#[test_case(4.0, 4.0, 0 => true)]
	#[test_case(4.001, 4.002, 0 => true)]
	#[test_case(4.001, 4.002, 1 => true)]
	#[test_case(4.001, 4.002, 2 => true)]
	#[test_case(4.001, 4.002, 3 => false)]
	#[test_case(4.001, 4.002, 4 => false)]
	#[test_case(241.79, 241.789, 2 => true)]
	fn approx_equal(a: f64, b: f64, decimal_places: u8) -> bool {
		super::approx_equal(a, b, decimal_places)
	}

	#[test_case(4.001, 0 => 4.0)]
	#[test_case(4.001, 1 => 4.0)]
	#[test_case(4.001, 2 => 4.0)]
	#[test_case(4.001, 3 => 4.001)]
	#[test_case(4.001, 4 => 4.001)]
	fn round_to_decimals(a: f64, places: i32) -> f64 {
		super::round_to_decimals(a, places)
	}

	#[test_case(4.001 => 4.0)]
	#[test_case(4.011 => 4.01)]
	#[test_case(4.099 => 4.10)]
	fn round_to_currency(a: f64) -> f64 {
		super::round_to_currency(a)
	}

	#[test_case(10000.00, 0.00625, 48, 241.79)] // 7.5% annual
	#[test_case(12000.00, 0.01083, 36, 404.33)] // 13% annual
	#[test_case(12000.00, 0.02083, 36, 477.12)] // 25% annual
	fn calculate_payment_amount(i: f64, r: f64, n: i64, expected: f64) {
		let loan = Loan {
			initial_value: i,
			rate: r,
			number_of_payments: n,
			payment_amount: 0.0, // value doesn't matter
			name: "".to_owned(), // value doesn't matter
		};

		let calculated = super::round_to_currency(loan.calculate_payment_amount());
		assert!(
			super::within_five_cents(expected, calculated),
			"Expected {} to be within 5 cents of {}",
			calculated,
			expected
		);
	}
}
