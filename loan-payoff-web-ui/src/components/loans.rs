use crate::components::LoanRow;
use loan_payoff::{Loan, pay_loans_all_orderings};
use web_sys::{HtmlInputElement, InputEvent};
use yew::prelude::*;
use yew::virtual_dom::VChild;

pub enum LoansMsg {
    AddLoan,
    Calculate,
    UpdateExtraAmount(String),
}

pub struct Loans {
    loans: Vec<Loan>,
    extra_amount: f64,
    optimal_payoff_display: String,
}

impl Component for Loans {
    type Message = LoansMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            loans: vec![
                Loan {
                    name: "eek".to_owned(),
                    initial_value: 1000.00,
                    rate: 0.023,
                    number_of_payments: 23,
                    // TODO: test with bad values
                    payment_amount: 56.47,
                },
                Loan {
                    name: "num2".to_owned(),
                    initial_value: 10000.00,
                    rate: 0.00625,
                    number_of_payments: 48,
                    // TODO: test with bad values: payment_amount: 234.43
                    payment_amount: 241.79,
                },
            ],
            // TODO: this should be set via ui as well
            extra_amount: 100.0,
            optimal_payoff_display: "".to_owned()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LoansMsg::Calculate => {
                // TODO: self.optimal_payoff_display = "".to_owned(), then continue
                let mut loans = Vec::new();
                for loan in self.loans.iter() {
                    loans.push(loan);
                }
                let optimal_payoff = pay_loans_all_orderings(&loans, self.extra_amount);

                let stra = format!(
                    "Best ordering = {}, with savings ${}",
                    optimal_payoff.ordering.iter().map(|&i| loans[i].name.clone()).collect::<Vec<String>>().join(" -> "),
                    optimal_payoff.savings
                );
                self.optimal_payoff_display = stra;

                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            },
            LoansMsg::AddLoan => {
                let loan3 = Loan {
                    name: "num3".to_owned(),
                    initial_value: 10000.00,
                    rate: 0.014,
                    number_of_payments: 48,
                    payment_amount: 287.52
                };
                self.loans.push(loan3);
                true
            },
            LoansMsg::UpdateExtraAmount(content) => {
                // TODO: if empty this throws, need to allow empty but disallow non-numerical
                self.extra_amount = content.parse::<f64>().unwrap();
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let items: Vec<VChild<LoanRow>> = self
            .loans
            .iter()
            .map(|item| {
                html_nested! {
                    <LoanRow loan={item.clone()} />
                }
            }).collect();

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
                        value={self.extra_amount.clone().to_string()}
                    />
                    <label for="extra_payment" class="active">{ "Extra Payment" }</label>
                </div>
                { "loans:" }
                <div class="row">
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
