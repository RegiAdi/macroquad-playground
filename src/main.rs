use macroquad::prelude::*;

const MOVEMENT_SPEED: f32 = 10.0;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        clear_background(GRAY);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        if is_key_down(KeyCode::Up) {
            draw_text("UP ^", 100.0, 50.0, 100.0, RED
            );
            y -= MOVEMENT_SPEED;
        }

        if is_key_down(KeyCode::Down) {
            draw_text("DOWN ^", 100.0, 50.0, 100.0, RED);
            y += MOVEMENT_SPEED;
        }

        if is_key_down(KeyCode::Right) {
            draw_text(
                "RIGHT ->", 
                screen_width() / 2.0,
                screen_height() / 2.0, 
                100.0, 
                RED
            );

            x += MOVEMENT_SPEED;
        }

        if is_key_down(KeyCode::Left) {
            draw_text(
                "LEFT <-", 
                screen_width() / 2.0,
                screen_height() / 2.0, 
                100.0, 
                RED
            );
            x -= MOVEMENT_SPEED;
        }

        draw_circle(x, y, 30.0, RED);

        next_frame().await;
    }
}
