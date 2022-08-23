use yew::{
    events::{ InputEvent, KeyboardEvent, MouseEvent },
    function_component, classes, 
    Properties, html, Callback, Component, Context, Html
};
//use wasm_bindgen::JsCast;
//use wasm_bindgen::closure::Closure;
//use wasm_bindgen::UnwrapThrowExt;
extern crate web_sys;
use yew::NodeRef;

mod utils;

enum Msg {
    Input(InputEvent),
    Press(KeyboardEvent),
    Click(char)
}

struct Comp{
    board: Vec<Vec<NodeRef>>
}

const KEYBOARD_0: [char; 10] = ['Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P'];
const KEYBOARD_1: [char; 9] = ['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L'];
const KEYBOARD_2: [char; 7] = ['Z', 'X', 'C', 'V', 'B', 'N', 'M'];

#[derive(Properties, PartialEq)]
pub struct KeybrButtonProps {
    pub onclick: Callback<MouseEvent>,
    pub character: char // needed key_col
}

#[function_component(KeybrButton)]
pub fn keybr_button(props: &KeybrButtonProps) -> Html {
    html! {
        //<button style={format!("background-color:{}","yellow")}>
        <button class={"keybr-button"} onclick={&props.onclick}>
        {
            //props.character.or(Some(' ')).unwrap()
            props.character
        }
        </button>
    }
}

// Keyboard viewing function
fn keyarr2html<T: yew::Component>(arr: &'static [char], ctx: &Context<T>) -> Html
where <T as yew::Component>::Message: From <Msg> {
    html! {
        <div class={classes!("keybr_row")}>
        {
            arr.iter().map(|c| html! {
                <KeybrButton character={*c} onclick={
                    &ctx.link().callback(|_: MouseEvent| Msg::Click(*c))
                    // Callback::from(|_: MouseEvent| callback.emit(Msg::Click(*c)))
                }/>
            }).collect::<Html>()
        }
        </div>
    }
}

impl Component for Comp {

    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { board: vec![vec![NodeRef::default(); utils::LEN]; utils::ROUNDS] }
    }

    /*fn rendered(&mut self, ctx: &Context<Self>, first_renderer: bool){
        if !first_renderer {
            return;
        }
        /*
        let document = gloo::utils::document();
        let listener = EventListener::new(&document, "keydown", |event| {
            let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
            // TODO: try to reuse the key
            // ctx.link().callback(|_: KeyboardEvent| Msg::Press(event.clone()));
        });
        self.kbd_listener.replace(listener);
        */
    }*/

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg{
            Msg::Input(input) => {
                match input.input_type().as_str() {
                    "insertText" => log::info!("insert: {}",input.data().unwrap()),
                    "deleteContentBackward" => log::info!("Backspace"),
                    e => panic!("Unknown type: {}",e)
                }
            }
            Msg::Press(event) => {
                log::info!("Pressed: {}", event.key());
                if event.key() == "Enter" {
                }
            }
            Msg::Click(c) => {
                log::info!("Clicked: {}", c);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = &ctx.link().callback(|event: InputEvent| {
            Msg::Input(event)
        });
        let onkeypress = &ctx.link().callback(|event: KeyboardEvent| {
            Msg::Press(event)
        });
        //let checkfocus = &ctx.link().callback(|e: KeyboardEvent| {});
        let keybr_r0 = keyarr2html(&KEYBOARD_0, ctx);
        let keybr_r1 = keyarr2html(&KEYBOARD_1, ctx);
        let keybr_r2 = keyarr2html(&KEYBOARD_2, ctx);

        html! {
            <h1 style="text-align:center">
            <div class={"board"}>
            {
                self.board.iter().enumerate().map(|(row, x)| html! {
                    <div class={"row"}>
                    {
                        x.iter().enumerate().map(|(col, x)| html! {
                            <input class={"tile"} maxlength={1} {onkeypress} {oninput}/>
                        }).collect::<Html>()
                    }
                    </div>
                }).collect::<Html>()
            }
            </div>
            <p>{ keybr_r0 }</p>
            <p>{ keybr_r1 }</p>
            <p>{ keybr_r2 }</p>
            </h1>
        }
    }
}

fn main(){
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Comp>();
}

