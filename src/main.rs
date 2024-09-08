use macroquad::prelude::*;

pub enum GameState {
    TitleScreen,
    LoadingSave,
    UiInteraction,
}

#[macroquad::main("Silli RPG")]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);

        draw_text("Hey stinky", 0.0, 20.0, 16.0, RED);

        next_frame().await
    }
}
