use loan_payoff::{self, Loan};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew_agent::{Agent, AgentLink, Context, HandlerId};

use super::loans::LoanViewModel;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
	AddLoan,
	Bump, // just used to get initial load
	DeleteLoan(usize),
	UpdateInitialValue(f64, usize),
	UpdateInterestRate(f64, usize),
	UpdateName(String, usize),
	UpdateNumberOfPayments(i64, usize),
}

pub struct EventBus {
	link: AgentLink<EventBus>,
	subscribers: HashSet<HandlerId>,
	loans: Vec<LoanViewModel>,
	current_key: i64,
}

impl Agent for EventBus {
	type Reach = Context<Self>;
	type Message = ();
	type Input = Request;
	type Output = Vec<LoanViewModel>;

	fn create(link: AgentLink<Self>) -> Self {
		Self {
			link,
			subscribers: HashSet::new(),
			current_key: 0,
			loans: Vec::new(),
		}
	}

	fn update(&mut self, _msg: Self::Message) {}

	fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
		match msg {
			Request::AddLoan => {
				let new_loan = LoanViewModel {
					loan: Loan {
						..Default::default()
					},
					key: self.current_key,
				};
				self.current_key += 1;
				self.loans.push(new_loan);
			}
			Request::DeleteLoan(index) => {
				self.loans.remove(index);
			}
			Request::Bump => { /* just responds below */ }
			Request::UpdateInitialValue(new_amount, index) => {
				self.loans[index].loan.initial_value = new_amount;
				let calculated_payment_amount = loan_payoff::round_to_currency(
					self.loans[index].loan.calculate_payment_amount(),
				);
				if calculated_payment_amount > 0.0 {
					self.loans[index].loan.payment_amount = calculated_payment_amount;
				} else {
					// TODO: set error flag and mark row as invalid
				}
			}
			Request::UpdateInterestRate(new_rate, index) => {
				self.loans[index].loan.rate = new_rate;
				let calculated_payment_amount = loan_payoff::round_to_currency(
					self.loans[index].loan.calculate_payment_amount(),
				);
				if calculated_payment_amount > 0.0 {
					self.loans[index].loan.payment_amount = calculated_payment_amount;
				} else {
					// TODO: set error flag and mark row as invalid
				}
			}
			Request::UpdateName(new_name, index) => {
				self.loans[index].loan.name = new_name;
			}
			Request::UpdateNumberOfPayments(new_number_of_payments, index) => {
				self.loans[index].loan.number_of_payments = new_number_of_payments;
				let calculated_payment_amount = loan_payoff::round_to_currency(
					self.loans[index].loan.calculate_payment_amount(),
				);
				if calculated_payment_amount > 0.0 {
					self.loans[index].loan.payment_amount = calculated_payment_amount;
				} else {
					// TODO: set error flag and mark row as invalid
				}
			}
		}

		for sub in self.subscribers.iter() {
			self.link.respond(*sub, self.loans.clone());
		}
	}

	fn connected(&mut self, id: HandlerId) {
		self.subscribers.insert(id);
	}

	fn disconnected(&mut self, id: HandlerId) {
		self.subscribers.remove(&id);
	}
}
