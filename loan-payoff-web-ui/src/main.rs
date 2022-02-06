use components::Loans;
use yew::prelude::*;

mod components;

struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        //let link = ctx.link();
        html! {
            <div>
                <Loans />
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    let document = gloo_utils::document();
    let mount_point = document.query_selector("div.mountpoint").unwrap().unwrap();
    yew::start_app_in_element::<Model>(mount_point);
}
