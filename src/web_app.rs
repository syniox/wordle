use yew::{
    events::{ InputEvent, KeyboardEvent, MouseEvent },
    function_component, classes, 
    Properties, html, Callback, Component, Context, Html,
    NodeRef
};
use wasm_bindgen::JsCast;
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
    fn get_elm((a, b): (usize, usize)) -> HtmlInputElement {
        let window = web_sys::window().expect("should have a window in this context");
        let document = window.document().expect("window should have a document");
        document.query_selector(&format!("#tile-{}{}", a, b))
            .unwrap().unwrap()
            .dyn_into::<HtmlInputElement>().unwrap()
    }
    fn get_focus_elm(&self) -> HtmlInputElement {
        //log::info!("get focus elm: {:?}", self.focus);
        //self.get_focus_ref().cast::<HtmlInputElement>()
        Self::get_elm(self.focus)
    }
    fn apply_focus(&self){
        /*if let Some(input) = self.get_focus_elm() {
            log::info!("try to focus on {:?}", self.focus);
            match input.focus(){
                Ok(_) => (),
                Err(e) => log::error!("Error: {:?}", e)
            }//.expect(&format!("focus_next error: {:?}", self.focus));
        } else {
            panic!("cannot get focused elm");
        }*/
        let elm = self.get_focus_elm();
        elm.focus().unwrap();
    }
    fn focus_next(&mut self, enter: bool) {
        if self.focus.1 < utils::LEN - 1 {
            self.focus.1 += 1;
        } else if enter == true {
            self.focus.1 = 0;
            self.focus.0 += 1;
            if self.focus.0 == utils::ROUNDS {
                return;
            }
        }
        //self.apply_focus();
    }
    fn focus_prev(&mut self) {
        if self.focus.1 > 0 {
            self.focus.1 -= 1;
            //self.apply_focus();
        }
    }

    pub fn backspace(&mut self){
        //let mut elm = self.get_focus_elm().unwrap();
        let mut elm = self.get_focus_elm();
        log::info!("backspace on {:?}", self.focus);
        log::info!("node_value:{}", elm.value());
        if elm.value().is_empty() {
            self.focus_prev();
            //elm = self.get_focus_elm().unwrap();
            elm = self.get_focus_elm();
        }
        if self.focus != (0, 0) {
            assert!(!elm.value().is_empty());
        }
        elm.set_value("");
    }
    pub fn linebreak(&mut self){
        if self.focus.1 != utils::LEN - 1 {
            return;
        }
        if self.get_focus_elm().value().is_empty() {
            return;
        }
        /* let guess = self.board[self.focus.0].iter()
            .map(|x| {
                let node = x.get().unwrap();
                assert!(node.node_value().unwrap().len() == 1);
                node.node_value().unwrap().pop().unwrap()
            })
            .collect::<String>();*/
        let guess = (0..utils::LEN).map(|x| {
            let mut elm = Self::get_elm((self.focus.0, x));
            assert!(elm.value().len()==1);
            elm.value().pop().unwrap()
        }).collect::<String>();
        log::info!("submit guess: {}", guess);
        // TODO: connect with game and find out whether focus next
    }
    pub fn insert(&mut self, _c: char) {
        if self.focus.1 != utils::LEN - 1 {
            self.focus_next(false);
        }
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

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
        }
        //self.apply_focus();
        let window = web_sys::window().expect("should have a window in this context");
        let document = window.document().expect("window should have a document");
        log::info!("trying to get tile-{}{}",self.focus.0,self.focus.1);
        document.query_selector(&format!("#tile-{}{}",self.focus.0,self.focus.1))
            .unwrap().unwrap()
            .dyn_ref::<HtmlInputElement>()
            .unwrap().focus().unwrap();
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
                self.focus = (2, 2);
                //self.insert(c);
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
                            id = {format!("tile-{}{}",row,col)}
                            />
                        }).collect::<Html>()
                    } </div>
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
