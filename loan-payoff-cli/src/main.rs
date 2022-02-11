use loan_payoff::{pay_loans_all_orderings, round_to_currency, Loan};
use log;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

fn main() {
	simple_logger::init_with_level(log::Level::Info).unwrap();

	let file_path = match env::args_os().nth(1) {
		None => {
			log::error!("expected at least 1 argument (file path, extra amount), but got none");
			process::exit(1);
		}
		Some(file_path) => file_path,
	};

	let extra_amount = match env::args().nth(2) {
		None => {
			log::trace!("no amount supplied for second argument, using default 100.00");
			100.0
		}
		Some(extra_amount) => match extra_amount.parse::<f64>() {
			Err(_) => {
				log::error!("could not parse entered value '{}' to f64", extra_amount);
				process::exit(1);
			}
			Ok(extra_amount) => round_to_currency(extra_amount),
		},
	};

	if let Err(err) = process_loans(file_path, extra_amount) {
		println!("error running example: {}", err);
		process::exit(1);
	}
}

fn process_loans(file_path: OsString, extra_amount: f64) -> Result<(), Box<dyn Error>> {
	let mut loans = Vec::new();
	let file = File::open(file_path)?;
	let mut rdr = csv::Reader::from_reader(file);
	for result in rdr.deserialize() {
		let loan: Loan = result?;
		println!("Read {}", loan);
		loans.push(loan);
	}

	let optimal_payoff = pay_loans_all_orderings(&loans.iter().map(|l| l).collect(), extra_amount)
		.expect("Failed to pay loans");

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

	Ok(())
}
