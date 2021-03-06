use super::event_bus::{EventBus, Request};
use loan_payoff::{self, Loan};
use std::fmt::{Debug, Display};
use web_sys::{HtmlInputElement, InputEvent};
use yew::prelude::*;
use yew_agent::{Dispatched, Dispatcher};

pub enum LoanMsg {
	Delete(usize),
	UpdateInitialValue(f64, usize),
	UpdateInterestRate(f64, usize),
	UpdateName(String, usize),
	UpdateNumberOfPayments(i64, usize),
}

pub struct LoanRow {
	event_bus: Dispatcher<EventBus>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct LoanProps {
	pub loan: Loan,
	pub index: usize,
	pub show_validation_errors: bool,
}

impl Component for LoanRow {
	type Message = LoanMsg;
	type Properties = LoanProps;

	fn create(_ctx: &Context<Self>) -> Self {
		Self {
			event_bus: EventBus::dispatcher(),
		}
	}

	fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
		match msg {
			LoanMsg::Delete(index) => {
				self.event_bus.send(Request::DeleteLoan(index));
				true
			}
			LoanMsg::UpdateInitialValue(new_amount, index) => {
				self.event_bus
					.send(Request::UpdateInitialValue(new_amount, index));
				true
			}
			LoanMsg::UpdateInterestRate(new_rate, index) => {
				self.event_bus
					.send(Request::UpdateInterestRate(new_rate, index));
				true
			}
			LoanMsg::UpdateName(new_name, index) => {
				self.event_bus.send(Request::UpdateName(new_name, index));
				true
			}
			LoanMsg::UpdateNumberOfPayments(new_number_of_payments, index) => {
				self.event_bus.send(Request::UpdateNumberOfPayments(
					new_number_of_payments,
					index,
				));
				true
			}
		}
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		let link = ctx.link();
		let mut calculated_payment_amount = ctx.props().loan.calculate_payment_amount();
		calculated_payment_amount = loan_payoff::round_to_currency(calculated_payment_amount);
		let index = ctx.props().index;
		let show_validation_errors = ctx.props().show_validation_errors;
		let input_class = if show_validation_errors
			&& (calculated_payment_amount.is_nan() || calculated_payment_amount.is_infinite())
		{
			"invalid".clone()
		} else {
			"".clone()
		};
		let name_class = if show_validation_errors && ctx.props().loan.name.is_empty() {
			"invalid".clone()
		} else {
			"".clone()
		};

		html! {
			<div class="row">
				<div class="col l2 s12">
					<div class="input-field">
						<input
							type="text"
							id="loan_name"
							class={name_class}
							oninput={link.callback(move |event: InputEvent| {
								let input: HtmlInputElement = event.target_unchecked_into();
								LoanMsg::UpdateName(input.value(), index)
							})}
							value={ctx.props().loan.name.clone()}
						/>
						<label for="loan_name" class="active hide-on-med-and-up">{ "Name" }</label>
					</div>
				</div>
				<div class="col l2 s12">
					<InputNumber<f64>
						value={ctx.props().loan.initial_value}
						{index}
						{input_class}
						id="loan_initial_value"
						label="Loan Amount"
						request={link.callback(move |new_val: f64| LoanMsg::UpdateInitialValue(new_val, index))}
					/>
				</div>
				<div class="col l2 s12">
					<InputNumber<f64>
						value={loan_payoff::round_to_decimals(ctx.props().loan.rate * 12.0 * 100.0, 1)}
						step=".1"
						{index}
						{input_class}
						id="loan_interest_rate"
						label="Interest Rate"
						request={link.callback(move |new_val: f64| LoanMsg::UpdateInterestRate(new_val/12.0/100.0, index))}
					/>
				</div>
				<div class="col l2 s12">
					<InputNumber<i64>
						value={ctx.props().loan.number_of_payments}
						{index}
						{input_class}
						id="loan_number_of_payments"
						label="Number of Payments"
						request={link.callback(move |new_val: i64| LoanMsg::UpdateNumberOfPayments(new_val, index))}
					/>
				</div>
				<div class="col l2 s12">
					<span class="hide-on-med-and-up">{ "Monthly Payment: " }</span>
					{ calculated_payment_amount }
				</div>
				<div class="col l2 s12">
					<span class="btn" onclick={link.callback(move |_| LoanMsg::Delete(index))}>
						<i class="small material-icons">{ "delete_forever" }</i>
					</span>
				</div>
			</div>
		}
	}
}

pub enum InputNumberMsg {
	UpdateValue(String),
}

#[derive(Clone, PartialEq, Properties)]
pub struct InputNumberProps<T: PartialEq> {
	pub value: T,
	pub index: usize,
	pub request: Callback<T>,
	pub id: String,
	pub label: String,
	#[prop_or("1".to_owned())]
	pub step: String,
	#[prop_or("".to_owned())]
	pub input_class: String,
}

pub struct InputNumber<T> {
	pub value: String,
	pub initial_value: T,
}

impl<T: Copy + 'static> Component for InputNumber<T>
where
	T: PartialEq + Display + std::str::FromStr + Debug,
	<T as std::str::FromStr>::Err: Debug,
{
	type Message = InputNumberMsg;
	type Properties = InputNumberProps<T>;

	fn create(ctx: &Context<Self>) -> Self {
		let value = ctx.props().value.to_string();

		Self {
			value,
			initial_value: ctx.props().value,
		}
	}

	fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
		match msg {
			InputNumberMsg::UpdateValue(new_value) => {
				let new_amount = new_value.parse::<T>();
				self.value = new_value;
				match new_amount {
					Ok(new_amount) => {
						ctx.props().request.emit(new_amount);
					}
					Err(e) => {
						// TODO: handle error better
						log::error!("Bad parse {:?}", e);
					}
				}
				true
			}
		}
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		let link = ctx.link();
		html! {
			<div class="input-field">
				<input
					type="number"
					step={ctx.props().step.clone()}
					class={ctx.props().input_class.clone()}
					id={ctx.props().id.clone()}
					oninput={link.callback(move |event: InputEvent| {
						let input: HtmlInputElement = event.target_unchecked_into();
						InputNumberMsg::UpdateValue(input.value())
					})}
					value={self.value.clone()}
				/>
				<label for={ctx.props().id.clone()} class="active hide-on-med-and-up">{ctx.props().label.clone()}</label>
			</div>
		}
	}
}
