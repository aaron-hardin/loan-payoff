use super::event_bus::{EventBus, Request};
use loan_payoff::{Loan, round_to_currency};
use web_sys::{HtmlInputElement, InputEvent};
use yew::prelude::*;
use yew_agent::{Dispatched, Dispatcher};

pub enum LoanMsg {
    Delete(i64),
    UpdateName(String, usize),
}

pub struct LoanRow {
    event_bus: Dispatcher<EventBus>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct LoanProps {
    pub loan: Loan,
    pub index: i64,
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
                self.event_bus.send(Request::DeleteLoan(index as usize));
                true
            },
            LoanMsg::UpdateName(new_name, index) => {
                self.event_bus.send(Request::UpdateName(new_name, index));
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
                <div class="col l2">
                    <div class="input-field">
                        <input
                            type="text"
                            id="loan_name"
                            oninput={link.callback(move |event: InputEvent| {
                                let input: HtmlInputElement = event.target_unchecked_into();
                                LoanMsg::UpdateName(input.value(), index as usize)
                            })}
                            value={ctx.props().loan.name.clone()}
                        />
                        // TODO: disabling label for now because it looks funny, need to revisit <label for="loan_name" class="active">{ "Name" }</label>
                    </div>
                </div>
                <div class="col l2">{ ctx.props().loan.initial_value.clone() }</div>
                <div class="col l2">{ ctx.props().loan.rate.clone() }</div>
                <div class="col l2">{ ctx.props().loan.number_of_payments.clone() }</div>
                <div class="col l2">{ calculated_payment_amount }</div>
                <div class="col l2">
                    <span class="btn" onclick={link.callback(move |_| LoanMsg::Delete(index))}>
                        <i class="small material-icons">{ "delete_forever" }</i>
                    </span>
                </div>
            </div>
        }
    }
}
