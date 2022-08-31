use gloo::timers::callback::Interval;
use yew::prelude::*;
mod life;
use life::*;

pub struct App {
    universe: Universe,
    _standalone: Interval,
}

#[derive(Properties, PartialEq)]
pub struct AppProps {
    uni_size: u32,
}

pub enum AppMsg {
    Tick,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = AppProps;

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let tick = Interval::new(10, move || link.send_message(AppMsg::Tick));

        Self {
            universe: Universe::new(ctx.props().uni_size),
            _standalone: tick,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::Tick => self.universe.tick(),
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <span class="body">{self.universe.to_string()}</span>
            </>
        }
    }
}

fn main() {
    let props = AppProps { uni_size: 64 };
    yew::start_app_with_props::<App>(props);
}
