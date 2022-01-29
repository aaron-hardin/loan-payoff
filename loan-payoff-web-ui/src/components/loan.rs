use loan_payoff::{Loan, round_to_currency};
use yew::prelude::*;

pub struct LoanRow;

#[derive(Clone, PartialEq, Properties)]
pub struct LoanProps {
    pub loan: Loan,
}

impl Component for LoanRow {
    type Message = ();
    type Properties = LoanProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut calculated_payment_amount = ctx.props().loan.calculate_payment_amount();
        calculated_payment_amount = round_to_currency(calculated_payment_amount);

        html! {
            <div class="row">
                <div class="col l2">{ format!("Loan: {}", calculated_payment_amount) }</div>
                <div class="col l2">{ ctx.props().loan.name.clone() }</div>
                <div class="col l2">{ ctx.props().loan.initial_value.clone() }</div>
                <div class="col l2">{ ctx.props().loan.rate.clone() }</div>
                <div class="col l2">{ ctx.props().loan.number_of_payments.clone() }</div>
                <div class="col l2">{ ctx.props().loan.payment_amount.clone() }</div>
            </div>
        }
    }
}
