mod bgsp_data;
use bgsp_data::*;

mod direction;
use direction::*;

mod input_role;
use input_role::*;

mod game_window;
use game_window::*;

mod wait_and_update;

use bgsp_lib2::{
    bgsp_common::*,
    bg_plane::*,
    sp_resources::*,
};

/*
use audio_lib::{
    AudioContext,
    Control,
    AUDIO_HALT_DATA,
};
*/

// use once_cell::sync::OnceCell;
use piston_window::{Key, ControllerButton, ControllerHat, HatState};

const FULL_SCREEN: bool = false;
const VM_RECT_SIZE: (i32, i32) = (48, 60);
const VM_RECT_PIXEL_SIZE: (i32, i32) = (VM_RECT_SIZE.0 * PATTERN_SIZE as i32, VM_RECT_SIZE.1 * PATTERN_SIZE as i32);
const ROTATION: Direction = Direction::Up;
const PIXEL_SCALE: i32 = 2;
const WINDOW_MARGIN: i32 = 2;
const BG0_RECT_SIZE: (i32, i32) = (80, 80);
const BG1_RECT_SIZE: (i32, i32) = (160, 160);
const MAX_SPRITES: usize = 512;
// const AUDIO_VOLUME: u16 = 5;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    // let audio_subsystem = sdl_context.audio().unwrap();
    let mut game_window = GameWindow::new(
        video_subsystem,
        FULL_SCREEN,
        VM_RECT_PIXEL_SIZE,
        ROTATION,
        PIXEL_SCALE,
        WINDOW_MARGIN,
    );

    let mut keyboard_map = InputRoleMap::<Key>::new();
    {
        let set_list = [
            (Key::D1,    InputRole::ViewRotLeft),
            (Key::D2,    InputRole::ViewRotRight),
            (Key::D3,    InputRole::Start),
            (Key::D4,    InputRole::Pause),
            (Key::Z,     InputRole::MainFire),
            (Key::X,     InputRole::SubFire),
            (Key::C,     InputRole::MainFire),
            (Key::Space, InputRole::MainFire),
            (Key::Space, InputRole::SubFire),
            (Key::W,     InputRole::Up),
            (Key::D,     InputRole::Right),
            (Key::S,     InputRole::Down),
            (Key::A,     InputRole::Left),
            (Key::Up,    InputRole::Up),
            (Key::Right, InputRole::Right),
            (Key::Down,  InputRole::Down),
            (Key::Left,  InputRole::Left),
        ];
        keyboard_map.assign(&set_list);
    }
    let mut button_map = InputRoleMap::<ControllerButton>::new();
    {
        let set_list = [
            (ControllerButton {id: 0, button: 0}, InputRole::MainFire),
            (ControllerButton {id: 0, button: 1}, InputRole::SubFire),
            (ControllerButton {id: 0, button: 2}, InputRole::MainFire),
            (ControllerButton {id: 0, button: 3}, InputRole::MainFire),
            (ControllerButton {id: 0, button: 3}, InputRole::SubFire),
        ];
        button_map.assign(&set_list);
    }
    let mut hat_map = InputRoleMap::<ControllerHat>::new();
    {
        let set_list = [
            (ControllerHat {id: 0, which: 0, state: HatState::Centered}, InputRole::None),
            (ControllerHat {id: 0, which: 0, state: HatState::Up}, InputRole::Up),
            (ControllerHat {id: 0, which: 0, state: HatState::Down}, InputRole::Down),
            (ControllerHat {id: 0, which: 0, state: HatState::Right}, InputRole::Right),
            (ControllerHat {id: 0, which: 0, state: HatState::Left}, InputRole::Left),
            (ControllerHat {id: 0, which: 0, state: HatState::RightUp}, InputRole::Right),
            (ControllerHat {id: 0, which: 0, state: HatState::RightUp}, InputRole::Up),
            (ControllerHat {id: 0, which: 0, state: HatState::RightDown}, InputRole::Right),
            (ControllerHat {id: 0, which: 0, state: HatState::RightDown}, InputRole::Down),
            (ControllerHat {id: 0, which: 0, state: HatState::LeftUp}, InputRole::Left),
            (ControllerHat {id: 0, which: 0, state: HatState::LeftUp}, InputRole::Up),
            (ControllerHat {id: 0, which: 0, state: HatState::LeftDown}, InputRole::Left),
            (ControllerHat {id: 0, which: 0, state: HatState::LeftDown}, InputRole::Down),
        ];
        hat_map.assign(&set_list);
    }
    let mut input_role_state = InputRoleState::default();

    let mut bg_texture_bank = BgTextureBank::new(
        &bgchar_data::BG_PATTERN_TBL,
        &bgpal_data::COLOR_TBL,
        game_window.pixel_scale() as i32,
    );
    let rc_bg_texture_bank = Rc::new(RefCell::new(&mut bg_texture_bank));
    let mut bg = {
        let bg0 = BgPlane::new(
            BG0_RECT_SIZE,
            VM_RECT_PIXEL_SIZE,
            rc_bg_texture_bank.clone(),
        );

        let bg1 = BgPlane::new(
            BG1_RECT_SIZE,
            VM_RECT_PIXEL_SIZE,
            rc_bg_texture_bank.clone(),
        );
        (bg0, bg1)
    };

    let mut sp_texture_bank = SpTextureBank::new(
        &spchar_data::SP_PATTERN_TBL,
        &sppal_data::COLOR_TBL,
        game_window.pixel_scale() as i32,
    );
    let rc_sp_texture_bank = Rc::new(RefCell::new(&mut sp_texture_bank));
    let mut spr = SpResources::new(
        MAX_SPRITES,
        rc_sp_texture_bank.clone(),
    );

    let text_buf = {
        let mut buf = Vec::new();
        let text_path = std::env::current_dir().unwrap().join("src").join("main.rs");
        for line in std::fs::read_to_string(text_path).unwrap().lines() {
            buf.push(line.to_string());
        }
        buf.push("[eof]------------------------------------------------------------------".to_string());
        buf
    };

    if game_window.full_screen() {
        sdl_context.mouse().show_cursor(false);
    }

    let mut t_count = 0;
    let mut scroll_pos = 0;
    // let mut main_state = 0;

    let (mut my_x256, mut my_y256) = (160 * 256, 320 * 256);
    let mut my_tilt = 0;
    let (mut v_x, mut v_y) = (0, 0);

    let mut shots: Vec<Option<((i32, i32), (i32, i32), SpCode)>> = Vec::with_capacity(16);
    let mut unused: Vec<usize> = Vec::with_capacity(16);
    let mut sub_weapon_heat = 0;
    let mut sub_weapon_cool = 0;

    {
        let s = "Test for shoot".to_string();
        let x = (VM_RECT_SIZE.0 - s.len() as i32) / 2;
        bg.0.set_cur_pos(x, 20)
            .put_string(&s, Some(&CharAttributes::new(3, BgSymmetry::Normal)));
    }
    spr.sp[0].code( 6).palette(1).symmetry(SpSymmetry::Normal); // PLAYER FIGHTER
    spr.sp[1].code(17).palette(3).symmetry(SpSymmetry::Normal); // BACK FIRE
    spr.sp[2].code(17).palette(3).symmetry(SpSymmetry::Normal); // BACK FIRE
    bg.0.set_cur_pos(0, 0).put_achar_n(&AChar::new(' ', 1, BgSymmetry::Normal), VM_RECT_SIZE.0);
    bg.0.set_cur_pos(0, 1).put_achar_n(&AChar::new(' ', 1, BgSymmetry::Normal), VM_RECT_SIZE.0);
    bg.1.fill_attributes(&CharAttributes::new(5, BgSymmetry::Normal));

    input_role_state.clear_all();
    'main_loop: loop {
        if scroll_pos % 8 == 0 {
            let len = text_buf.len();
            let bgpos_y = (scroll_pos / 8) - 1;
            let text_line = ((len as i32 + bgpos_y) % len as i32 + len as i32) % len as i32;
            bg.0.set_cur_pos(0, 1)
                .put_string(&format!("{:3} {:3}/{:3} ", bgpos_y, text_line, len), None);
            bg.1.set_cur_pos(0, bgpos_y)
                .put_string(&text_buf[text_line as usize], None);
            let remains = BG1_RECT_SIZE.0 - bg.1.cur_pos().0;
            if remains > 0 {
                bg.1.put_code_n(' ', remains);
            }
        }
        bg.1.set_view_pos(my_x256 / 512, scroll_pos);
        scroll_pos -= 1;

        bg.0.set_cur_pos(25,0)
            .put_string(&format!("({:3}, {:3})", my_x256 / 256, my_y256 / 256), None)
            .put_code(if t_count % 4 < 2 { ' ' } else { '*' })
            .set_cur_pos(4,0)
            .put_string(&format!("{:3}[{:3}]", shots.len(), unused.len()), None);

        match (input_role_state.get(InputRole::Up).0, input_role_state.get(InputRole::Down).0) {
            (true, false) => {
                if v_y > 0 { v_y = 0; } else {
                    v_y -= 256;
                    if v_y < -512 { v_y = -512; }
                }
            }
            (false, true) => {
                if v_y < 0 { v_y = 0; } else {
                    v_y += 256;
                    if v_y > 512 { v_y = 512; }
                }
            }
            _ => {}
        }
        match (input_role_state.get(InputRole::Left).0, input_role_state.get(InputRole::Right).0) {
            (true, false) => {
                if v_x > 0 { v_x = 0; } else {
                    v_x -= 256;
                    if v_x < -768 { v_x = -768; }
                }
                my_tilt -= 2;
                if my_tilt < -34 { my_tilt = -34; }
            }
            (false, true) => {
                if v_x < 0 { v_x = 0; } else {
                    v_x += 256;
                    if v_x > 768 { v_x = 768; }
                }
                my_tilt += 2;
                if my_tilt > 34 { my_tilt = 34; }
            }
            _ => {}
        }

        if v_x != 0 {
            v_x += if v_x < 0 { 128 } else { -128 };
        }
        if v_y != 0 {
            v_y += if v_y < 0 { 128 } else { -128 };
        }

        my_x256 += v_x;
        my_y256 += v_y;
        if my_x256 < -10 * 256 { my_x256 = -10 * 256; }
        if my_x256 > 329 * 256 { my_x256 = 329 * 256; }
        if my_y256 < 100 * 256 { my_y256 = 100 * 256; }
        if my_y256 > 412 * 256 { my_y256 = 412 * 256; }
        if my_tilt != 0 {
            my_tilt += if my_tilt < 0 { 1 } else { -1 };
        }
        {
            let (my_code, drift, l_offset, r_offset) = match my_tilt {
                -40..=-29 => ( 0, -1, 23, 36),
                -28..=-22 => ( 1, -1, 23, 36),
                -21..=-15 => ( 2, -1, 22, 36),
                -14..=-8  => ( 3, -1, 22, 37),
                 -7..=-3  => ( 4,  0, 21, 37),
                 -2..=-1  => ( 5,  0, 21, 37),
                  1..=2   => ( 7,  0, 21, 37),
                  3..=7   => ( 8,  0, 21, 37),
                  8..=14  => ( 9,  1, 21, 36),
                 15..=21  => (10,  1, 22, 36),
                 22..=28  => (11,  1, 22, 35),
                 29..=40  => (12,  1, 22, 35),
                _         => ( 6,  0, 21, 37)
            };
            let my_pos = SpPos::new(my_x256 / 256 + drift, my_y256 / 256);
            spr.sp(0).pos(my_pos).code(my_code).visible(true);
            let back_fire = {
                let th = if v_y == 0 { 25 } else if v_y < 0 { 15 } else { 45 };
                let n = 17 + (t_count as u32 % th) / 2;
                if n > 25 { 17 } else { n }
            };
            let y_offset = 65;
            let vis = t_count % 2 == 0;
            spr.sp(1).xy(my_pos.x + l_offset, my_pos.y + y_offset).code(back_fire).visible(vis);
            spr.sp(2).xy(my_pos.x + r_offset, my_pos.y + y_offset).code(back_fire).visible(vis);
        }
        {
            const SHOT_PAT_NO: SpCode = 15;
            const SUBSHOT_PAT_NO: SpCode = 13;
            if input_role_state.get(InputRole::MainFire).0 {
                let (dx, dy) = {
                //    let t = (t_count % 16) - 8;
                //    if t < 0 { -t * 128 } else { t * 128 }
                    ((t_count % 8) * 200 ,-14 * 256 + 96)
                };
                let new_shot = Some(((my_x256 + 21 * 256, my_y256 + 10 * 256), (-dx, dy), SHOT_PAT_NO));
                if let Some(i) = unused.pop() {
                    shots[i] = new_shot;
                } else {
                    shots.push(new_shot);
                }
                let new_shot = Some(((my_x256 + 35 * 256, my_y256 + 10 * 256), (dx, dy), SHOT_PAT_NO));
                if let Some(i) = unused.pop() {
                    shots[i] = new_shot;
                } else {
                    shots.push(new_shot);
                }
                /*
                let new_shot = Some(((my_x256 + 28 * 256, my_y256 - 14 * 256), (0, dy), SHOT_PAT_NO));
                if let Some(i) = unused.pop() {
                    shots[i] = new_shot;
                } else {
                    shots.push(new_shot);
                }
                */
            }
            if input_role_state.get(InputRole::SubFire).0 && sub_weapon_cool <= 0 {
                let new_shot = Some(((my_x256 + 24 * 256, my_y256 + 10 * 256), (0, -32 * 256), SUBSHOT_PAT_NO));
                if let Some(i) = unused.pop() {
                    shots[i] = new_shot;
                } else {
                    shots.push(new_shot);
                }
                let new_shot = Some(((my_x256 + 24 * 256, my_y256 - 6 * 256), (0, -32 * 256), SUBSHOT_PAT_NO));
                if let Some(i) = unused.pop() {
                    shots[i] = new_shot;
                } else {
                    shots.push(new_shot);
                }
                sub_weapon_heat += 1;
                if sub_weapon_heat > 8 {
                    sub_weapon_cool = 12;
                }
            } else {
                if sub_weapon_heat > 0 {
                    sub_weapon_heat -= 1;
                }
                if sub_weapon_cool > 0 {
                    sub_weapon_cool -= 1;
                }
            }
            let mut sp_idx = 16;
            for i in 0..shots.len() {
                if let Some(((x256, y256), (dx, dy), c)) = shots[i] {
                    spr.sp(sp_idx).xy(x256 / 256, y256 /256).code(c).palette(1).visible(true);
                    let new_x256 = x256 + dx;
                    let new_y256 = y256 + dy;
                    if new_y256 < -32 * 256 || new_x256 < -16 * 256 || new_x256 > 388 * 256 {
                        shots[i] = None;
                        unused.push(i);
                    } else {
                        shots[i] = Some(((new_x256, new_y256), (dx, dy), c));
                    }
                } else {
                    spr.sp(sp_idx).visible(false);
                }
                sp_idx += 1;
            }
        }
        if input_role_state.get(InputRole::ViewRotLeft).1 & 0b1111 == 0b0011 { // Release -> Press
            game_window.turn_left();
        }
        if input_role_state.get(InputRole::ViewRotRight).1 & 0b1111 == 0b1100 { // Press -> Release
            game_window.turn_right();
        }
        if wait_and_update::doing(&mut game_window, &mut spr, &mut bg, &mut keyboard_map, &mut button_map, &mut hat_map) {
            break 'main_loop;
        }
        input_role_state.clear_state();
        input_role_state.update_state(&keyboard_map);
        input_role_state.update_state(&button_map);
        input_role_state.update_state(&hat_map);
        input_role_state.update_history();
        t_count += 1;
    }
    sdl_context.mouse().show_cursor(true);
}
