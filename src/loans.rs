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
                Loan { name: "eek".to_owned() },
                Loan { name: "num2".to_owned() }
            ],
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let items: Vec<VChild<Loan>> = self
            .loans
            .iter()
            .enumerate()
            .map(|(ix, item)| {
                html_nested! {
                    <Loan loan={item.clone()} />
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
}

#[derive(Clone, PartialEq, Properties)]
pub struct LoanProps {
    loan: Loan,
}

impl Component for Loan {
    type Message = ();
    type Properties = LoanProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            name: "".to_owned()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                { self.name.clone() }
            </div>
        }
    }
}
