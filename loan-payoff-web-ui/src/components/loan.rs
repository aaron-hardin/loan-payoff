use super::event_bus::{EventBus, Request};
use loan_payoff::{Loan, round_to_currency};
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
            },
            LoanMsg::UpdateInitialValue(new_amount, index) => {
                self.event_bus.send(Request::UpdateInitialValue(new_amount, index));
                true
            },
            LoanMsg::UpdateInterestRate(new_rate, index) => {
                self.event_bus.send(Request::UpdateInterestRate(new_rate, index));
                true
            },
            LoanMsg::UpdateName(new_name, index) => {
                self.event_bus.send(Request::UpdateName(new_name, index));
                true
            },
            LoanMsg::UpdateNumberOfPayments(new_number_of_payments, index) => {
                self.event_bus.send(Request::UpdateNumberOfPayments(new_number_of_payments, index));
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let mut calculated_payment_amount = ctx.props().loan.calculate_payment_amount();
        calculated_payment_amount = round_to_currency(calculated_payment_amount);
        let index = ctx.props().index;

        html! {
            <div class="row">
                <div class="col l2 s12">
                    <div class="input-field">
                        <input
                            type="text"
                            id="loan_name"
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
                        id="loan_initial_value"
                        label="Loan Amount"
                        request={link.callback(move |new_val: f64| LoanMsg::UpdateInitialValue(new_val, index))}
                    />
                </div>
                <div class="col l2 s12">
                    <InputNumber<f64>
                        value={ctx.props().loan.rate}
                        step=".001"
                        {index}
                        id="loan_interest_rate"
                        label="Interest Rate"
                        request={link.callback(move |new_val: f64| LoanMsg::UpdateInterestRate(new_val, index))}
                    />
                </div>
                <div class="col l2 s12">
                    <InputNumber<i64>
                        value={ctx.props().loan.number_of_payments}
                        {index}
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
}

pub struct InputNumber<T> {
    pub value: String,
    pub initial_value: T,
}

impl<T: Copy + 'static> Component for InputNumber<T>
 where
     T: PartialEq + Display + std::str::FromStr + Debug, <T as std::str::FromStr>::Err: Debug {
    type Message = InputNumberMsg;
    type Properties = InputNumberProps<T>;

    fn create(ctx: &Context<Self>) -> Self {
        let value = ctx.props().value.to_string();

        Self {
            value,
            initial_value: ctx.props().value
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
                    },
                    Err(e) => {
                        // TODO: handle error better
                        log::error!("Bad parse {:?}", e);
                    }
                }
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="input-field">
                <input
                    type="number"
                    step={ctx.props().step.clone()}
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
