use raylib::prelude::*;
use rand;
use std::time;
//use std::thread;

fn main() {
    let height = 720;
    let width = 1280;
    let tile_size = 10;
    let scaled_h:i32 = height/tile_size;
    let scaled_w:i32 = width/tile_size;
    let delay = time::Duration::from_millis(50);
    let mut now = time::Instant::now();
    let mut toggle = false;
    // x*w 
    let mut game_field = vec![false; (scaled_w*scaled_h) as usize];
    for i in 0..game_field.len() {
        game_field[i] = rand::random::<bool>();
        //println!("value: {field}");
    }

    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("Game of life")
        .build();
    while !rl.window_should_close() {

        let mut d =rl.begin_drawing(&thread);
        if d.is_key_pressed(raylib::ffi::KeyboardKey::KEY_SPACE) {
            toggle = !toggle;
        }

        if  toggle && now.elapsed() > delay || d.is_key_pressed(raylib::ffi::KeyboardKey::KEY_S){
            let mut v = game_field.clone();
            for i in 0..game_field.len() {
                let x = i as i32 % scaled_w;
                let y = i as i32 / scaled_w;
                let mut cnt = 0;
                for k in -1..2 {
                    for j in -1..2 {
                        if (x+j) >= 0  && (x+j) < scaled_w && (y+k) >= 0  && (y+k) < scaled_h {
                            if game_field[((x+j) + scaled_w*(y+k)) as usize] && i != ((x+j) + scaled_w*(y+k)) as usize {
                                cnt += 1;
                            } 
                        }
                    }
                }

                if game_field[i] {
                    if cnt > 3  || cnt < 2{
                        v[i] = false;
                    }

                } else {
                    if cnt == 3 {
                        v[i] = true;
                    }
                }
            }
            game_field = v;
            now = time::Instant::now();
        }

        if d.is_key_pressed(raylib::ffi::KeyboardKey::KEY_R) {
            for i in 0..game_field.len() {
                game_field[i] = rand::random::<bool>();
                //println!("value: {field}");
            }
        
        }

        if d.is_key_pressed(raylib::ffi::KeyboardKey::KEY_C) {
            toggle = false;
            for i in 0..game_field.len() {
                game_field[i] = false;
                //println!("value: {field}");
            }
        
        }

        if d.is_mouse_button_down(raylib::ffi::MouseButton::MOUSE_LEFT_BUTTON) {
            let x = d.get_mouse_x()/tile_size;
            let y = d.get_mouse_y()/tile_size;
            if x >= 0 && x < scaled_w && y >= 0 && y < scaled_h{
                game_field[(x+ y*scaled_w) as usize] = true;
            }
            
        }

        if d.is_mouse_button_down(raylib::ffi::MouseButton::MOUSE_RIGHT_BUTTON) {
            let x = d.get_mouse_x()/tile_size;
            let y = d.get_mouse_y()/tile_size;
            if x >= 0 && x < scaled_w && y >= 0 && y < scaled_h{
                game_field[(x+ y*scaled_w) as usize] = false;
            }
            
        }

        d.clear_background(Color::BLACK);
        for y in 0..scaled_h {
            for x in 0..scaled_w {
                if game_field[(x+ y*scaled_w) as usize] {
                    d.draw_rectangle(x*tile_size, y*tile_size, tile_size, tile_size, Color::WHITE);
                }
            }
        }
    }
    
}
