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
    Input(InputEvent, usize, usize), // row, col
    Press(KeyboardEvent, usize, usize), // row, col
    Click(char),
}

struct App{
    board: Vec<Vec<NodeRef>>,
    focus: (usize, usize)
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

impl Component for App {

    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            board: vec![vec![NodeRef::default(); utils::LEN]; utils::ROUNDS],
            focus: (0, 0)
        }
    }

    /*fn rendered(&mut self, ctx: &Context<Self>, first_renderer: bool){
        if !first_renderer { return; }
        let document = gloo::utils::document();
        let listener = EventListener::new(&document, "keydown", |event| {
            let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
            // TODO: try to reuse the key
            // ctx.link().callback(|_: KeyboardEvent| Msg::Press(event.clone()));
        });
        self.kbd_listener.replace(listener);
    }*/

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg{
            Msg::Input(input, r, c) => {
                log::info!("typed on row {r} col {c}");
                match input.input_type().as_str() {
                    "insertText" => log::info!("insert: {}",input.data().unwrap()),
                    "deleteContentBackward" => log::info!("Backspace"), // please don't do anything here
                    e => panic!("Unknown type: {}",e)
                }
            }
            Msg::Press(event, r, c) => {
                log::info!("Pressed: {}", event.key());
                if event.key() == "Enter" {

                }
                if event.key() == "Backspace" {

                }
            }
            Msg::Click(c) => {
                log::info!("Clicked: {}", c);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = |row, col| {ctx.link().batch_callback(move |event: InputEvent| {
            let mut s = event.data().unwrap_or(String::new());
            if s.is_empty() || (s.len() == 1 && s.pop().unwrap().is_alphabetic()) {
                Some(Msg::Input(event, row, col))
            } else {
                event.prevent_default();
                None
            }
        })};
        let onkeydown = |row, col| {ctx.link().callback(move |event: KeyboardEvent| {
            if event.key() == "Enter" || event.key() == "Backspace" {
                event.prevent_default();
            }
            Msg::Press(event, row, col)
        })};
        let keybr_r0 = keyarr2html(&KEYBOARD_0, ctx);
        let keybr_r1 = keyarr2html(&KEYBOARD_1, ctx);
        let keybr_r2 = keyarr2html(&KEYBOARD_2, ctx);

        html! {
            <h1 style="text-align:center">
            // Dashboard
            <div class={"board"}> {
                self.board.iter().enumerate().map(|(row, x)| html! {
                    <div class={"row"}> {
                        x.iter().enumerate().map(|(col, _)| html! {
                            <input class={"tile"}
                            maxlength={1}
                            onkeydown = {onkeydown(row,col)}
                            oninput = {oninput(row, col)}
                            />
                        }).collect::<Html>()
                    }
                    </div>
                }).collect::<Html>()
            }
            </div>
            // Keyboard
            <p>{ keybr_r0 }</p>
            <p>{ keybr_r1 }</p>
            <p>{ keybr_r2 }</p>
            </h1>
        }
    }
}

fn main(){
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
