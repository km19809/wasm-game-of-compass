mod color;
mod shape;
mod utils;
#[macro_use]
mod text;
use color::Color;
use shape::{Circle, Position2d, Rect};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

impl From<Color> for JsValue {
    fn from(s: Color) -> JsValue {
        s.to_string().into()
    }
}

trait Drawable {
    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, fill_style: &JsValue) -> ();
}

impl Drawable for Circle {
    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, fill_style: &JsValue) -> () {
        context.begin_path();
        let pos = self.position();
        context
            .arc(
                pos.x.round(),
                pos.y.round(),
                self.radius().round(),
                0.0,
                std::f64::consts::PI * 2.0,
            )
            .unwrap();
        context.set_fill_style(fill_style);
        context.fill();
        context.close_path();
    }
}
#[derive(Debug, Clone)]
struct Player {
    name: &'static str,
    color: Color,
    button: web_sys::HtmlButtonElement,
    active: bool,
    score: u32,
}

impl Player {
    fn new(name: &'static str, color: Color, button: web_sys::HtmlButtonElement) -> Player {
        Player {
            name,
            color,
            button,
            active: true,
            score: 0,
        }
    }
    fn deactivate(&mut self) {
        self.active = false;
        self.set_text(&format!(TEMPLATE_TEXT_GIVEN_UP!(), self.score));
        self.set_disabled(true);
    }
    fn color(&self) -> &Color {
        &self.color
    }
    fn set_text(&mut self, msg: &str) {
        self.button
            .set_inner_text(&format!("{}: {}", self.name, msg));
    }
    fn set_disabled(&mut self, disabled: bool) {
        self.button.set_disabled(disabled);
    }
    fn active(&self) -> bool {
        self.active
    }
    fn score(&self) -> u32 {
        self.score
    }
    fn add_score(&mut self, amount: u32) -> u32 {
        self.score += amount;
        self.score
    }
}

fn validate(
    board: &Rect,
    circles: &[Circle],
    current_circle: &Circle,
    min_radius: f64,
    max_radius: f64,
) -> Result<(), &'static str> {
    let radius = current_circle.radius();
    if circles.iter().any(|c| c.is_overlapped(current_circle)) {
        Err("Overlapped circle")
    } else if board.is_outside(current_circle) {
        Err("Outside of the board")
    } else if min_radius > radius {
        Err("Smaller than the minimum limit.")
    } else if radius > max_radius {
        Err("Larger than the maximum limit.")
    } else {
        Ok(())
    }
}
/// Returns:\
/// `Some(usize)` Index of the next active player. (include themselves.)
/// `None` if none of the players active.
fn next_player_idx(players: &[Player], current_idx: Option<usize>) -> Option<usize> {
    if let Some(idx) = current_idx {
        let number_of_players = players.len();
        for i in 0..number_of_players {
            let next_idx = (idx + 1 + i) % number_of_players;
            if players[next_idx].active() {
                return Some(next_idx);
            }
        }
    }
    None
}

fn disable_other_players(players: &mut [Player], current_idx: Option<usize>) {
    for player in players.iter_mut() {
        player.set_disabled(true); //Enable the 1st player only.
    }
    if let Some(idx) = current_idx {
        players[idx].set_disabled(false);
    }
}

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
    fn cancelInterval(token: f64);

    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn stylish_log(msg: &str, style: &str);
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    utils::set_panic_hook();
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .expect("Need an canvase element with id=\"cnavas\".")
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    let context = Rc::new(context);
    //Iint Players
    let mut players = Vec::with_capacity(4);
    let red_button = document
        .get_element_by_id("player_red")
        .expect("Need an canvase element with id=\"player_red\".")
        .dyn_into::<web_sys::HtmlButtonElement>()?;
    players.push(Player::new("R", Color::Red, red_button));
    let green_button = document
        .get_element_by_id("player_green")
        .expect("Need an canvase element with id=\"player_green\".")
        .dyn_into::<web_sys::HtmlButtonElement>()?;
    players.push(Player::new("G", Color::Green, green_button));
    let blue_button = document
        .get_element_by_id("player_blue")
        .expect("Need an canvase element with id=\"player_blue\".")
        .dyn_into::<web_sys::HtmlButtonElement>()?;
    players.push(Player::new("B", Color::Blue, blue_button));
    let yellow_button = document
        .get_element_by_id("player_yellow")
        .expect("Need an canvase element with id=\"player_yellow\".")
        .dyn_into::<web_sys::HtmlButtonElement>()?;
    players.push(Player::new("Y", Color::Yellow, yellow_button));
    disable_other_players(&mut players, Some(0));
    //Global game contexts
    let players = Rc::new(RefCell::new(players));
    let current_player_idx = Rc::new(Cell::new(Some(0)));
    let circles = Rc::new(RefCell::new(Vec::<Circle>::new()));
    let circle_fill_styles = Rc::new(RefCell::new(Vec::<Color>::new()));
    let current_circle = Rc::new(RefCell::new(Circle::new(0.0, 0.0, 0.0)));
    let current_circle_valid = Rc::new(Cell::new(true));
    let width = 1280;
    let height = 720;
    canvas.set_width(width);
    canvas.set_height(height);
    let client_to_canvas_width: f64 = width as f64 / canvas.client_width() as f64;
    let client_to_canvas_height: f64 = height as f64 / canvas.client_height() as f64;
    log(&format! {"w {}h {}",client_to_canvas_width,client_to_canvas_height});
    let (min_radius, max_radius) = (18.0, 360.0);
    let board = Rect::new(0.0, 0.0, width.into(), height.into());
    let pressed = Rc::new(Cell::new(false));

    //on:mousedown
    {
        let pressed = pressed.clone();
        let circle = current_circle.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            circle.borrow_mut().set_position(
                event.offset_x() as f64 * client_to_canvas_width,
                event.offset_y() as f64 * client_to_canvas_height,
            );
            pressed.set(true);
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    //on:mousemove
    {
        let pressed = pressed.clone();
        let circles = circles.clone();
        let circle = current_circle.clone();
        let current_circle_valid = current_circle_valid.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if pressed.get() {
                let center = circle.borrow().position();
                {
                    circle.borrow_mut().set_radius(
                        center
                            .distance(&Position2d {
                                x: event.offset_x() as f64 * client_to_canvas_width,
                                y: event.offset_y() as f64 * client_to_canvas_height,
                            })
                            .floor(),
                    );
                }
                let valid = match validate(
                    &board,
                    &circles.borrow(),
                    &circle.borrow(),
                    min_radius,
                    max_radius,
                ) {
                    Ok(_) => true,
                    Err(msg) => {
                        log(msg);
                        false
                    }
                };
                current_circle_valid.set(valid);
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    //on:mouseup
    {
        let pressed = pressed.clone();
        let players = players.clone();
        let current_player_idx = current_player_idx.clone();
        let circles = circles.clone();
        let circle_fill_styles = circle_fill_styles.clone();
        let current_circle = current_circle.clone();
        let current_circle_valid = current_circle_valid.clone();
        let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            let valid = match validate(
                &board,
                &circles.borrow(),
                &current_circle.borrow(),
                min_radius,
                max_radius,
            ) {
                Ok(_) => true,
                Err(msg) => {
                    log(msg);
                    false
                }
            };
            current_circle_valid.set(valid);
            if valid {
                let area = current_circle.borrow().area();
                log(&format!("{}", area));
                {
                    circles.borrow_mut().push(current_circle.take());
                }
                let current_player_idx_copy = current_player_idx.get();
                if let Some(idx) = current_player_idx_copy {
                    let current_player_color = *players.borrow()[idx].color();
                    {
                        let score = players.borrow_mut()[idx].add_score(area as u32);
                        {
                            players.borrow_mut()[idx].set_text(&format!("{}", score));
                        }
                        stylish_log(
                            &format!("Player %c ★ {}", score),
                            &format!("color:{};", current_player_color),
                        );
                    }
                    {
                        circle_fill_styles.borrow_mut().push(current_player_color);
                    }
                }
                let next_player_idx = next_player_idx(&players.borrow(), current_player_idx_copy);
                current_player_idx.set(next_player_idx);
                disable_other_players(&mut players.borrow_mut(), next_player_idx);
            } else {
                let circle = current_circle.take();
                log(&format!("{:?} is invalid", circle));
            }
            pressed.set(false);
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    //on:mouseleave
    {
        let pressed = pressed.clone();
        let circle = current_circle.clone();
        let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            log(&format!("Mouse leaved while drawing {:?}", circle.take()));
            pressed.set(false);
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mouseleave", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    //Buttons on:click
    {
        let players = players.clone();
        for player in players.borrow_mut().iter_mut() {
            let players = players.clone();
            let current_player_idx = current_player_idx.clone();
            let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
                let current_idx = current_player_idx.get();
                if let Some(idx) = current_idx {
                    players.borrow_mut()[idx].deactivate();
                }
                let next_player_idx = next_player_idx(&players.borrow(), current_idx);
                if next_player_idx.is_none() {
                    //GameEnd;
                    players.borrow_mut().sort_by_key(|p| p.score());
                }
                current_player_idx.set(next_player_idx);
                disable_other_players(&mut players.borrow_mut(), next_player_idx)
            }) as Box<dyn FnMut(_)>);
            player
                .button
                .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
    }
    //Draw loop
    {
        let players = players.clone();
        let current_player_idx = current_player_idx.clone();
        let circle_fill_styles = circle_fill_styles.clone();
        let circles = circles.clone();
        let current_circle = current_circle.clone();
        let current_circle_valid = current_circle_valid.clone();
        let context = context.clone();
        let closure = Closure::wrap(Box::new(move || {
            context.clear_rect(0.0, 0.0, width.into(), height.into());
            for (c, fill_style) in circles
                .borrow()
                .iter()
                .zip(circle_fill_styles.borrow().iter())
            {
                c.draw(&context, &(*fill_style).into());
            }
            if let Some(idx) = current_player_idx.get() {
                let current_player_color = *players.borrow()[idx].color();
                {
                    let fill_style = if current_circle_valid.get() {
                        current_player_color.to_dark()
                    } else {
                        current_player_color.to_light()
                    };
                    current_circle.borrow().draw(&context, &fill_style.into());
                }
            } else {
                //No active player means the game is finished.
                let center: f64 = width as f64 / 2.0f64;
                let line_space: f64 = 50.0;
                let middle: f64 =
                    (height as f64 - line_space * (players.borrow().len() + 1) as f64) / 2.0;
                context.set_font("50px system-ui");
                context.set_stroke_style(&"#000000".to_string().into());
                context.set_line_width(4.0);
                for (i, player) in players.borrow().iter().rev().enumerate() {
                    context.set_fill_style(&(*player.color()).into());
                    if i == 0 {
                        let msg = &format!(TEMPLATE_TEXT_WINNER!(), player.name);
                        context
                            .stroke_text(msg, center, middle)
                            .expect("Failed to stroke text.");
                        context
                            .fill_text(msg, center, middle)
                            .expect("Failed to print text.");
                    }
                    let msg = &format!(TEMPLATE_TEXT_RANKING!(), i + 1, player.name, player.score);
                    context
                        .stroke_text(msg, center, middle + line_space * (i as f64 + 1.0))
                        .expect("Failed to stroke text.");
                    context
                        .fill_text(msg, center, middle + line_space * (i as f64 + 1.0))
                        .expect("Failed to print text.");
                }
                context.set_text_align("center");
                context.set_text_baseline("middle");
            }
        }) as Box<dyn FnMut()>);
        window.set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            20,
        )?;
        closure.forget();
    }

    Ok(())
}
