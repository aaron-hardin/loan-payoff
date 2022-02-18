use super::event_bus::{EventBus, Request};
use crate::components::LoanRow;
use loan_payoff::{pay_loans_all_orderings, Loan};
use web_sys::{HtmlInputElement, InputEvent};
use yew::prelude::*;
use yew::virtual_dom::VChild;
use yew_agent::{Bridge, Bridged, Dispatched, Dispatcher};

pub enum LoansMsg {
	AddLoan,
	Calculate,
	UpdateExtraAmount(String),
	UpdateLoans(Vec<LoanViewModel>),
}

#[derive(Clone)]
pub struct LoanViewModel {
	pub key: i64, // Unique id to use as a key
	pub loan: Loan,
}

pub struct Loans {
	loans: Vec<LoanViewModel>,
	extra_amount: f64,
	optimal_payoff_display: String,
	event_bus: Dispatcher<EventBus>,
	_producer: Box<dyn Bridge<EventBus>>,
	show_validation_errors: bool,
}

impl Component for Loans {
	type Message = LoansMsg;
	type Properties = ();

	fn create(ctx: &Context<Self>) -> Self {
		let mut me = Self {
			loans: Vec::new(),
			// TODO: this should be set via ui as well
			extra_amount: 100.0,
			optimal_payoff_display: "".to_owned(),
			event_bus: EventBus::dispatcher(),
			_producer: EventBus::bridge(ctx.link().callback(LoansMsg::UpdateLoans)),
			show_validation_errors: false,
		};

		me.event_bus.send(Request::Bump);
		me
	}

	fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
		match msg {
			LoansMsg::Calculate => {
				for loan in self.loans.iter() {
					if loan.loan.payment_amount.is_nan() || loan.loan.payment_amount.is_infinite()
					{
						self.show_validation_errors = true;
						return true;
					}
					if loan.loan.name.is_empty()
					{
						self.show_validation_errors = true;
						return true;
					}
				}

				// TODO: self.optimal_payoff_display = "".to_owned(), then continue
				let mut loans = Vec::new();
				for loan in self.loans.iter() {
					loans.push(&loan.loan);
				}
				match pay_loans_all_orderings(&loans, self.extra_amount) {
					Ok(optimal_payoff) => {
						self.show_validation_errors = false;
						let stra = format!(
							"Best ordering = {}, with savings ${}, is debt snowball {}, savings over debt snowball ${}",
							optimal_payoff.ordering.iter().map(|&i| loans[i].name.as_ref()).collect::<Vec<_>>().join(" -> "),
							optimal_payoff.savings,
							optimal_payoff.is_debt_snowball,
							optimal_payoff.savings_over_debt_snowball
						);
						self.optimal_payoff_display = stra;
					}
					Err(e) => {
						self.optimal_payoff_display = format!("Err {:?}", e);
					}
				}

				// the value has changed so we need to
				// re-render for it to appear on the page
				true
			}
			LoansMsg::AddLoan => {
				self.event_bus.send(Request::AddLoan);
				true
			}
			LoansMsg::UpdateExtraAmount(content) => {
				// TODO: if empty this throws, need to allow empty but disallow non-numerical
				self.extra_amount = content.parse::<f64>().unwrap();
				true
			}
			LoansMsg::UpdateLoans(loans) => {
				self.loans = loans;
				true
			}
		}
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		let link = ctx.link();
		let items: Vec<VChild<LoanRow>> = self
			.loans
			.iter()
			.enumerate()
			.map(|(index, item)| {
				html_nested! {
					<LoanRow key={item.key} loan={item.loan.clone()} index={index} show_validation_errors={self.show_validation_errors} />
				}
			})
			.collect();

		html! {
			<div>
				<div class="input-field">
					<input
						type="text"
						id="extra_payment"
						oninput={link.callback(|event: InputEvent| {
							let input: HtmlInputElement = event.target_unchecked_into();
							LoansMsg::UpdateExtraAmount(input.value())
						})}
						value={self.extra_amount.to_string()}
					/>
					<label for="extra_payment" class="active">{ "Extra Payment" }</label>
				</div>
				<div class="row hide-on-small-only">
					<div class="col l2">{ "Name" }</div>
					<div class="col l2">{ "Loan Amount" }</div>
					<div class="col l2">{ "Interest Rate" }</div>
					<div class="col l2">{ "Number of Payments" }</div>
					<div class="col l2">{ "Monthly Payment" }</div>
					<div class="col l2"></div>
				</div>
				{ items }
				<button onclick={link.callback(|_| LoansMsg::Calculate)} class="btn space-right">
					{ "Calculate" }
				</button>
				<button onclick={link.callback(|_| LoansMsg::AddLoan)} class="btn">
					{ "Add Loan" }
				</button>
				if !self.optimal_payoff_display.is_empty() {
					<div>
						{ self.optimal_payoff_display.clone() }
					</div>
				}
			</div>
		}
	}
}
