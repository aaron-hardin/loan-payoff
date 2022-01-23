use yew::prelude::*;
use yew::virtual_dom::VChild;

pub struct Loans {
    loans: Vec<Loan>,
}

impl Component for Loans {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            loans: vec![
                Loan {
                    name: "eek".to_owned(),
                    present_value: 1000.00,
                    rate: 1.23,
                    number_of_payments: 23,
                    payment_amount: 234.43
                },
                Loan {
                    name: "num2".to_owned(),
                    present_value: 10000.00,
                    rate: 0.00625,
                    number_of_payments: 48,
                    payment_amount: 234.43
                },
            ],
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let items: Vec<VChild<LoanRow>> = self
            .loans
            .iter()
            .enumerate()
            .map(|(ix, item)| {
                html_nested! {
                    <LoanRow loan={item.clone()} />
                }
            }).collect();

        html! {
            <div>
                { "loans:" }
                { items }
            </div>
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Loan {
    pub name: String,
    pub present_value: f64,
    pub rate: f64,
    pub number_of_payments: i64,
    pub payment_amount: f64,
}

impl Loan {
    fn calculate_payment_amount(&self) -> f64 {
        self.present_value * (self.rate * f64::powf(1.0+self.rate, self.number_of_payments as f64))
        /
        (f64::powf(1.0+self.rate, self.number_of_payments as f64) - 1.0)
    }
}

pub struct LoanRow;

#[derive(Clone, PartialEq, Properties)]
pub struct LoanProps {
    loan: Loan,
}

impl Component for LoanRow {
    type Message = ();
    type Properties = LoanProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let calculated_payment_amount = ctx.props().loan.calculate_payment_amount();

        html! {
            <div class="row">
                <div class="col l2">{ format!("Loan: {}", calculated_payment_amount) }</div>
                <div class="col l2">{ ctx.props().loan.name.clone() }</div>
                <div class="col l2">{ ctx.props().loan.present_value.clone() }</div>
                <div class="col l2">{ ctx.props().loan.rate.clone() }</div>
                <div class="col l2">{ ctx.props().loan.number_of_payments.clone() }</div>
                <div class="col l2">{ ctx.props().loan.payment_amount.clone() }</div>
            </div>
        }
    }
}
