use loan_payoff::{Loan};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew_agent::{Agent, AgentLink, Context, HandlerId};

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    AddLoan,
    Bump, // just used to get initial load
    DeleteLoan(usize),
}

pub struct EventBus {
    link: AgentLink<EventBus>,
    subscribers: HashSet<HandlerId>,
    loans: Vec<Loan>,
}

impl Agent for EventBus {
    type Reach = Context<Self>;
    type Message = ();
    type Input = Request;
    type Output = Vec<Loan>;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
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
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            Request::AddLoan => {
                let loan3 = Loan {
                    name: "num3".to_owned(),
                    initial_value: 10000.00,
                    rate: 0.014,
                    number_of_payments: 48,
                    payment_amount: 287.52
                };
                self.loans.push(loan3);
            },
            Request::DeleteLoan(index) => {
                self.loans.remove(index);
            },
            Request::Bump => { /* just responds below */ }
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