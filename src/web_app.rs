use yew::{
    events::{ InputEvent, KeyboardEvent, MouseEvent },
    function_component, classes, 
    Properties, html, Callback, Component, Context, Html,
    NodeRef
};
//use wasm_bindgen::JsCast;
//use wasm_bindgen::closure::Closure;
//use wasm_bindgen::UnwrapThrowExt;
extern crate web_sys;
use web_sys::HtmlInputElement;

mod utils;
mod game;
use game::{Game,Stats};

/*
mod args;
use args::Args;
*/

enum Msg {
    Input(InputEvent, usize, usize), // row, col
    Press(KeyboardEvent, usize, usize), // row, col
    Click(char),
}

struct App{
    game: Game,
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

impl App {
    // focus on the right thing
    fn get_focus_ref(&self) -> NodeRef {
        self.board[self.focus.0][self.focus.1].clone()
    }
    fn apply_focus(&self){
        if let Some(input) = self.get_focus_ref().cast::<HtmlInputElement>() {
            input.focus().expect(&format!("focus_next error: {:?}", self.focus));
        }
    }
    fn focus_next(&mut self, enter: bool) {
        if self.focus.1 < utils::ROUNDS - 1 {
            self.focus.1 += 1;
        } else if enter == true {
            self.focus.1 = 0;
            self.focus.0 += 1;
            if self.focus.0 == utils::ROUNDS {
                return;
            }
        }
        self.apply_focus();
    }
    fn focus_prev(&mut self) {
        if self.focus.1 > 0 {
            self.focus.1 -= 1;
            self.apply_focus();
        }
    }

    pub fn backspace(&mut self){
        let node = self.board[self.focus.0][self.focus.1].get().unwrap();
        log::info!("backspace on {:?}", self.focus);
        log::info!("node_value:{}", node.node_value().unwrap_or(String::from("None")));
        if node.node_value().is_none(){
            self.focus_prev();
        }
        if self.focus != (0, 0) {
            assert!(!node.node_value().unwrap().is_empty());
        }
        node.set_node_value(Some(""));
    }
    pub fn linebreak(&mut self){
        if self.focus.1 != utils::ROUNDS - 1 {
            return;
        }
        let guess = self.board[self.focus.0].iter()
            .map(|x| {
                let node = x.get().unwrap();
                assert!(node.node_value().unwrap().len() == 1);
                node.node_value().unwrap().pop().unwrap()
            })
            .collect::<String>();
        // TODO: connect with game
    }
    pub fn insert(&mut self, c: char) {
        //let node = self.get_focus_ref().get().unwrap();
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
            game: Game::new(),
            board: vec![vec![NodeRef::default(); utils::LEN]; utils::ROUNDS],
            focus: (0, 0)
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg{
            Msg::Input(input, r, c) => {
                log::info!("typed on row {r} col {c}");
                match input.input_type().as_str() {
                    "insertText" => {
                        let mut s = input.data().unwrap();
                        log::info!("insert: {}",s);
                        assert!(s.len() == 1);
                        self.insert(s.pop().unwrap());
                    }
                    _ => unreachable!()
                }
            }
            Msg::Press(event, r, c) => {
                log::info!("Pressed: {}", event.key());
                if event.key() == "Enter" {
                    self.linebreak();
                }
                if event.key() == "Backspace" {
                    self.backspace();
                }
            }
            Msg::Click(c) => {
                // TODO: backspace and enter
                log::info!("Clicked: {}", c);
                self.insert(c);
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
                            ref={self.board[row][col].clone()}
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
