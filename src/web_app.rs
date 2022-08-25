use std::default::Default;
use yew::{
    classes,
    events::{InputEvent, KeyboardEvent, MouseEvent},
    function_component, html, Callback, Component, Context, Html, NodeRef, Properties,
};
// use wasm_bindgen::JsCast;
// use wasm_bindgen::closure::Closure;
// use wasm_bindgen::UnwrapThrowExt;
extern crate web_sys;
use web_sys::HtmlInputElement;

mod game;
mod utils;
use game::{Game, Stats};

mod builtin_words;
mod words;

mod args;
use args::Args;

enum Msg {
    Input(InputEvent),
    Press(KeyboardEvent),
    Click(char),
    SwitchMode,
    Refresh,
    Reset,
}

// TODO Set the answer of game
struct App {
    game: Game,
    args: Args,
    stats: Stats,
    col_brd: Vec<Vec<i8>>,
    col_alpha: Vec<i8>,
    words: words::Words,
    board: Vec<Vec<NodeRef>>,
    focus: (usize, usize),
    hint: String,
}

const KEYBOARD_0: [char; 10] = ['Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P'];
const KEYBOARD_1: [char; 9] = ['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L'];
const KEYBOARD_2: [char; 7] = ['Z', 'X', 'C', 'V', 'B', 'N', 'M'];

fn id2background(id: i8) -> &'static str {
    match id {
        0 => "default",
        1 => "red",
        2 => "yellow",
        3 => "green",
        _ => unreachable!(),
    }
}

#[derive(Properties, PartialEq)]
pub struct KeybrButtonProps {
    pub onclick: Callback<MouseEvent>,
    pub character: String, // needed key_col
    pub key_col: &'static str,
}

#[function_component(KeybrButton)]
pub fn keybr_button(props: &KeybrButtonProps) -> Html {
    html! {
        <button class={"keybr-button"} onclick={&props.onclick}
        style={format!("background: {}", props.key_col)}>
        {
            &props.character
        }
        </button>
    }
}

impl App {
    // focus on the right thing
    fn get_focus_ref(&self) -> NodeRef {
        self.board[self.focus.0][self.focus.1].clone()
    }
    fn get_focus_elm(&self) -> HtmlInputElement {
        //log::info!("get focus elm: {:?}", self.focus);
        self.get_focus_ref().cast::<HtmlInputElement>().unwrap()
    }
    fn apply_focus(&self) {
        //log::info!("apply focus to {:?}", self.focus);
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
    }
    fn focus_prev(&mut self) {
        if self.focus.1 > 0 {
            self.focus.1 -= 1;
        }
    }
    fn disabled(&self, row: usize, col: usize) -> bool {
        return self.game.ended() || (row, col) != self.focus;
    }
    fn postproc(&mut self) {
        self.stats.store_game(self.game.clone());
    }

    pub fn start(&mut self) {
        // clear colors
        self.col_alpha.iter_mut().for_each(|col| *col = 0);
        self.col_brd.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|col| *col = 0);
        });
        // clear characters
        self.board.iter_mut().for_each(|row| {
            row.iter_mut()
                .for_each(|node| match node.cast::<HtmlInputElement>() {
                    None => log::info!("missing element"),
                    Some(elm) => elm.set_value(""),
                });
        });
        // ensure focus
        self.focus = (0, 0);

        if let Some(w) = self.args.word.as_ref() {
            log::info!("answer copied from {}", w);
            self.game.set_answer(w.clone());
        } else {
            let d = match self.args.day {
                None => {
                    log::warn!("day not initialized, initialize to 1");
                    1
                }
                Some(d) => d,
            };
            self.game = Game::new();
            let answer = self.words.final_list[d as usize].clone();
            self.game.set_answer(answer);
            self.args.day = Some(d + 1);
        }
        log::info!("game start: answer {}", self.game.show_answer());
    }
    pub fn insert(&mut self, _c: char) {
        if self.focus.1 != utils::LEN - 1 {
            self.focus_next(false);
        }
    }
    pub fn backspace(&mut self) {
        let mut elm = self.get_focus_elm();
        if self.game.ended() {
            return;
        }
        //log::info!("backspace on {:?}, value: {}", self.focus, elm.value());
        if elm.value().is_empty() {
            self.focus_prev();
            elm = self.get_focus_elm();
        }
        if self.focus.1 != 0 {
            assert!(!elm.value().is_empty());
        }
        elm.set_value("");
    }
    pub fn linebreak(&mut self) {
        // collect the word
        let guess = self.board[self.focus.0]
            .iter()
            .map(|x| x.cast::<HtmlInputElement>().unwrap())
            .filter(|n| n.value().len() == 1)
            .map(|n| n.value().pop().unwrap())
            .collect::<String>()
            .to_ascii_uppercase();
        // return if invalid
        log::info!("submit guess: {}", guess);
        if guess.len() < utils::LEN {
            self.hint = format!("Not enough letters: {}", guess);
            return;
        }
        if !self.words.valid.contains(&guess) {
            log::warn!("invalid words: {}", guess);
            self.hint = format!("{} isn't a word.", guess);
            return;
        }
        if self.args.difficult && !self.game.hard_check(&guess) {
            self.hint = format!("{}: must use information revealed before.", guess);
            return;
        }
        self.game.guess(guess);
        // colorize
        let (col_pos, col_alpha) = self.game.show_col();
        log::info!("color: {:?}", col_pos);
        self.col_brd[self.focus.0] = col_pos.clone();
        self.col_alpha = col_alpha.clone();
        // post-process
        if self.game.ended() {
            self.postproc();
        } else {
            self.focus_next(true);
        }
    }
}

// Keyboard viewing function
fn keyarr2html<T: yew::Component>(arr: &'static [char], col: &Vec<i8>, ctx: &Context<T>) -> Html
where
    <T as yew::Component>::Message: From<Msg>,
{
    html! {
        {
            arr.iter().map(|c| html! {
                <KeybrButton character={c.to_string()}
                    onclick={&ctx.link().callback(|_: MouseEvent| Msg::Click(*c))}
                    key_col={id2background(col[*c as usize - 'A' as usize])}
                    />
            }).collect::<Html>()
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut args: args::Args = Default::default();
        // Set random mode to true and add random seed since this is a web app
        args.random = true;
        if args.seed.is_none() {
            let mut s = [0u8];
            if let Err(e) = getrandom::getrandom(s.as_mut_slice()) {
                log::warn!("failed to get random seed: {}", e);
            }
            args.seed = Some(s[0].into());
        }
        let mut app = Self {
            game: Game::new(),
            stats: Default::default(),
            board: (0..utils::ROUNDS)
                .map(|_| (0..utils::LEN).map(|_| NodeRef::default()).collect())
                .collect(),
            words: words::Words::new(&args),
            args: args,
            col_brd: vec![vec![0i8; utils::LEN]; utils::ROUNDS],
            col_alpha: vec![0i8; 26],
            focus: (0, 0),
            hint: String::new(),
        };
        app.start();
        app
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        self.apply_focus();
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.hint = String::new();
        match msg {
            // Handle inputs expect backspace and Enter
            Msg::Input(input) => {
                match input.input_type().as_str() {
                    "insertText" => {
                        let mut s = input.data().unwrap();
                        //log::info!("insert: {}",s);
                        assert!(s.len() == 1);
                        self.insert(s.pop().unwrap());
                    }
                    s => log::warn!("Unused input_type: {}", s),
                }
            }
            // Handle backspace and enter
            Msg::Press(event) => {
                if event.key() == "Enter" {
                    //log::info!("Pressed: {}", event.key());
                    self.linebreak();
                }
                if event.key() == "Backspace" {
                    //log::info!("Pressed: {}", event.key());
                    self.backspace();
                }
            }
            // Handle on-screen keyboard inputs
            Msg::Click(c) => {
                log::info!("Clicked: {}", c);
                if c == '\x08' {
                    self.backspace();
                } else if c == '\n' {
                    self.linebreak();
                } else {
                    let elm = self.get_focus_elm();
                    if elm.value().is_empty() {
                        elm.set_value(&c.to_string());
                        self.insert(c);
                    } else {
                        assert!(self.focus.1 == utils::LEN - 1);
                    }
                }
            }
            Msg::SwitchMode => {
                if self.game.rounds() == 0 || self.game.ended() || self.args.difficult {
                    self.args.difficult ^= true;
                } else {
                    unreachable!();
                }
            }
            Msg::Refresh => (),
            Msg::Reset => self.start(),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Header helper
        let hard_invld_msg = "Hard mode can only be enabled at the start of a round.";
        // Board helper
        let oninput = &ctx.link().batch_callback(|event: InputEvent| {
            let mut s = event.data().unwrap_or(String::new());
            if s.is_empty() || (s.len() == 1 && s.pop().unwrap().is_alphabetic()) {
                Some(Msg::Input(event))
            } else {
                event.prevent_default();
                None
            }
        });
        let onkeydown = &ctx.link().callback(|event: KeyboardEvent| {
            if event.key() == "Enter" || event.key() == "Backspace" {
                event.prevent_default();
            }
            Msg::Press(event)
        });
        // Keybr helper
        let onclick = |c| ctx.link().callback(move |_| Msg::Click(c));
        let keybr_r0 = keyarr2html(&KEYBOARD_0, &self.col_alpha, ctx);
        let keybr_r1 = keyarr2html(&KEYBOARD_1, &self.col_alpha, ctx);
        let keybr_r2 = keyarr2html(&KEYBOARD_2, &self.col_alpha, ctx);
        // Stats helper
        let (win_rounds, lose_rounds, avg_guesses) = self.stats.feed_stats();
        let w_list = self.stats.feed_words();

        html! {
            <div style="text-align:center">
            // Menubar
            <p>
            if self.game.rounds() == 0 || self.game.ended() || self.args.difficult {
                <input type="checkbox" id="hardmode" checked={self.args.difficult} oninput={
                    ctx.link().callback(|_| Msg::SwitchMode)
                }/>
                <label for="hardmode">{"Hard mode"}</label>
            } else {
                <input type="checkbox" id="hardmode" checked={self.args.difficult}
                    disabled={true} title={hard_invld_msg}
                />
                <label for="hardmode" title={hard_invld_msg}>{"Hard mode"}</label>
            }
            </p>
            // Dashboard
            <div class={"board"}> {
                self.board.iter().enumerate().map(|(row, x)| html! {
                    <div class={"row"}> {
                        x.iter().enumerate().map(|(col, _)| html! {
                            <input class={"tile"}
                            ref={self.board[row][col].clone()}
                            maxlength={1}
                            onkeydown={onkeydown}
                            oninput={oninput}
                            onclick={ctx.link().callback(|_| Msg::Refresh)}
                            style={
                                format!("background: {};",
                                    id2background(self.col_brd[row][col])
                                )
                            }
                            disabled={self.disabled(row, col)}
                            />
                        }).collect::<Html>()
                    } </div>
                }).collect::<Html>()
            }
            </div>
            // Reset button
            if self.game.ended() {
                <button class={"keybr-button"} onclick={
                    ctx.link().callback(|_: MouseEvent| Msg::Reset)
                }>{"Restart!"}</button>
            }
            if !self.game.ended(){
                // Hint board
                <p style="white-space:pre">{format!("{} ", self.hint)}</p>
                // Keyboard
                <div class={classes!("keybr_row")}>
                { keybr_r0 }
                </div>
                <div class={classes!("keybr_row")}>
                { keybr_r1 }
                </div>
                <div class={classes!("keybr_row")}>
                <KeybrButton character="Enter" onclick={onclick('\n')} key_col={"grep"}/>
                { keybr_r2 }
                <KeybrButton character="Backspace" onclick={onclick('\x08')} key_col={"grep"}/>
                </div>
            }
            // Statistics
            if self.game.ended(){
                if self.game.won() {
                    <p style="color: green">{format!("Colgratulations!")}</p>
                } else {
                    <p>{format!("The correct answer is {}.", self.game.show_answer())}</p>
                }
                <p>{"Statictics:"}</p>
                <div style="display:inline-flex">
                <p style="margin:0.6em; color:green">{format!("Win: {}", win_rounds)}</p>
                <p style="margin:0.6em; color:red">{format!("Lose: {}", lose_rounds)}</p>
                <p style="margin:0.6em">{format!("AVG guesses: {:.2}", avg_guesses)}</p>
                </div>
                <p>{"Words used most:"}</p>
                <table style="" align="center">
                {
                    w_list.iter().enumerate().map(|(index, (word, times))| html!{
                        <tr>
                            <th><button disabled={true}>{format!("NO.{}",index+1)}</button></th>
                            <th>{format!("{}",times)}</th>
                            <th>{format!("{}",word)}</th>
                        </tr>
                    }).collect::<Html>()
                }
                </table>
            }
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
