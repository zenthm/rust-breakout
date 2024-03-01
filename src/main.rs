use macroquad::prelude::*;

#[macroquad::main("breakout")]
async fn main() {
    loop {
        clear_background(BLACK);
        next_frame().await
    }
}
