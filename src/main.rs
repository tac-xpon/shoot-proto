use piston_window::*;
use sdl2_window::Sdl2Window;
use std::collections::BTreeMap;
// use once_cell::sync::OnceCell;

use bgsp_lib2::bgsp_common::*;
use bgsp_lib2::bg_plane::*;
use bgsp_lib2::sp_resources::*;

mod bgsp_data;
use bgsp_data::*;
mod direction;
use direction::*;
mod input_role;
use input_role::*;
mod wait_and_update;

pub type GameWindow = piston_window::PistonWindow<sdl2_window::Sdl2Window>;

pub struct DisplayInfo {
    pub full_screen: bool,
    pub vm_rect_size: (i32, i32),
    pub rotation: direction::Direction,
    pub pixel_scale: i32,
    pub margin: i32,
    pub f_count: i32,
}

const FULL_SCREEN: bool = false;
const VM_RECT_SIZE: (i32, i32) = (48 * PATTERN_SIZE as i32, 60 * PATTERN_SIZE as i32);
const ROTATION: Direction = Direction::Up;
const PIXEL_SCALE: i32 = 2;
const WINDOW_MARGIN: i32 = 2;
const BG0_RECT_SIZE: (i32, i32) = (80, 80);
const BG1_RECT_SIZE: (i32, i32) = (160, 160);
const MAX_SPRITES: u32 = 512;
// const AUDIO_VOLUME: u16 = 5;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    // let audio_subsystem = sdl_context.audio().unwrap();
    let mut window: GameWindow;
    let mut display_info = {
        let full_screen = FULL_SCREEN;
        let vm_rect_size = VM_RECT_SIZE;
        let rotation = ROTATION;
        let pixel_scale = PIXEL_SCALE;
        let margin = WINDOW_MARGIN;
        let view_rect = {
            let (width, height) = (vm_rect_size.0 * pixel_scale, vm_rect_size.1 * pixel_scale);
            match rotation {
                Direction::Up    | Direction::Down => (width, height),
                Direction::Right | Direction::Left => (height, width),
            }
        };

        window = {
            const OPENGL_VER: OpenGL = OpenGL::V3_2;
            let window_title = format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            let window_rect_size = if full_screen {
                [8192, 8192]
            } else {
                [(view_rect.0 + margin * 2) as u32, (view_rect.1 + margin * 2) as u32]
            };
            let window_setting = WindowSettings::new(&window_title, window_rect_size)
                .samples(0)
                .fullscreen(full_screen)
                .exit_on_esc(true)
                .graphics_api(OpenGL::V3_2)
                .vsync(true)
                .resizable(false)
                .decorated(true)
                .controllers(true)
            ;
            let sdl2_window = Sdl2Window::with_subsystem(
                video_subsystem,
                &window_setting,
            ).unwrap();
            PistonWindow::new(OPENGL_VER, 0, sdl2_window)
        };
        window.set_max_fps(120);
        window.set_ups(60);
        window.set_ups_reset(0);
        window.set_swap_buffers(true);
        window.set_bench_mode(false);
        window.set_lazy(false);
        DisplayInfo {
            full_screen,
            vm_rect_size,
            rotation,
            pixel_scale,
            margin,
            f_count: 0,
        }
    };

    let mut keyboard_map: BTreeMap<piston_window::Key, Vec<_>> = BTreeMap::new();
    {
        let key_set_list = vec![
            (piston_window::Key::D1, InputRole::Button0),
            (piston_window::Key::D2, InputRole::Button1),
            (piston_window::Key::D3, InputRole::Button2),
            (piston_window::Key::D4, InputRole::Button3),
            (piston_window::Key::Z, InputRole::Button4),
            (piston_window::Key::X, InputRole::Button5),
            (piston_window::Key::C, InputRole::Button6),
            (piston_window::Key::Space, InputRole::Button7),
            (piston_window::Key::W, InputRole::Up),
            (piston_window::Key::D, InputRole::Right),
            (piston_window::Key::S, InputRole::Down),
            (piston_window::Key::A, InputRole::Left),
            (piston_window::Key::E, InputRole::Up),
            (piston_window::Key::E, InputRole::Right),
            (piston_window::Key::Up, InputRole::Up2),
            (piston_window::Key::Right, InputRole::Right2),
            (piston_window::Key::Down, InputRole::Down2),
            (piston_window::Key::Left, InputRole::Left2),
        ];
        for key_set in key_set_list {
            if let Some(role_list) = keyboard_map.get_mut(&key_set.0) {
                role_list.push(key_set.1);
            } else {
                keyboard_map.insert(key_set.0, vec![key_set.1]);
            }
        }
    }
    let mut input_role_state = InputRoleState::default();

    let mut bg = {
        let mut bg0 = BgPlane::new(
            BG0_RECT_SIZE,
            VM_RECT_SIZE,
            &bgchar_data::BG_CHARS,
            &bgpal_data::COLOR_TBL,
            display_info.pixel_scale,
        );
        bg0.set_base_symmetry(BgSymmetry::Normal);

        let mut bg1 = BgPlane::new(
            BG1_RECT_SIZE,
            VM_RECT_SIZE,
            &bgchar_data::BG_CHARS,
            &bgpal_data::COLOR_TBL,
            display_info.pixel_scale,
        );
        bg1.set_base_symmetry(BgSymmetry::Normal);
        (bg0, bg1)
    };

    let mut spr = {
        let mut sp: Vec<ClassicSprite> = Vec::with_capacity(MAX_SPRITES as usize);
        for _ in 0..MAX_SPRITES {
            sp.push(ClassicSprite { ..Default::default()});
        }
        let texture_bank = SpTextureBank::new(
            &spchar_data::SP_PATTERN_TBL,
            &sppal_data::COLOR_TBL,
            display_info.pixel_scale,
        );
        SpResources {
            sp,
            texture_bank,
            default_symmetry: SpSymmetry::Normal,
            pixel_scale: display_info.pixel_scale,
        }
    };

    if display_info.full_screen { sdl_context.mouse().show_cursor(false) }

    let mut t_count = 0;
    // let mut main_state = 0;

    let (mut my_x256, mut my_y256) = (160 * 256, 320 * 256);
    let mut my_tilt = 0;
    let (mut v_x, mut v_y) = (0, 0);

    let mut shots: Vec<Option<((i32, i32), (i32, i32), SpCode)>> = Vec::with_capacity(16);
    let mut unused: Vec<usize> = Vec::with_capacity(16);

    bg.0.set_cur_pos(20, 20)
        .put_string("Test for shoot", Some(&CharAttributes::new(2, BgSymmetry::Normal)));
    spr.sp[0].code(6).palette(1).symmetry(SpSymmetry::Normal);
    spr.sp[1].code(17).palette(2).symmetry(SpSymmetry::Normal);
    spr.sp[2].code(17).palette(2).symmetry(SpSymmetry::Normal);
    bg.0.set_cur_pos(0, 0).put_achar_n(&AChar::new(' ', 1, BgSymmetry::Normal), 80);

    input_role_state.clear_all();
    'main_loop: loop {
        bg.0.set_cur_pos(25,0)
            .put_string(&format!("({:3}, {:3})", my_x256 / 256, my_y256 / 256), None)
            .put_achar(&AChar::new('*', if t_count % 4 < 2 { 0 } else { 1 }, BgSymmetry::Normal))
            .set_cur_pos(4,0)
            .put_string(&format!("{:3}[{:3}]", shots.len(), unused.len()), None);
        if input_role_state.get(InputRole::Up2).0 {
            if v_y > 0 { v_y = 0 } else {
                v_y -= 256;
                if v_y < -512 { v_y = -512 }
            }
        }
        if input_role_state.get(InputRole::Down2).0 {
            if v_y < 0 { v_y = 0 } else {
                v_y += 256;
                if v_y > 512 { v_y = 512 }
            }
        }
        if input_role_state.get(InputRole::Left2).0 {
            if v_x > 0 { v_x = 0 } else {
                v_x -= 256;
                if v_x < -768 { v_x = -768 }
            }
            my_tilt -= 2;
            if my_tilt < -34 { my_tilt = -34 }
        }
        if input_role_state.get(InputRole::Right2).0 {
            if v_x < 0 { v_x = 0 } else {
                v_x += 256;
                if v_x > 768 { v_x = 768 }
            }
            my_tilt += 2;
            if my_tilt > 34 { my_tilt = 34 }
        }
        if v_x != 0 {
            v_x += if v_x < 0 { 128 } else { -128 }
        }
        if v_y != 0 {
            v_y += if v_y < 0 { 128 } else { -128 }
        }
        my_x256 += v_x;
        my_y256 += v_y;
        if my_x256 < -10 * 256 { my_x256 = -10 * 256 }
        if my_x256 > 329 * 256 { my_x256 = 329 * 256 }
        if my_y256 < 100 * 256 { my_y256 = 100 * 256 }
        if my_y256 > 412 * 256 { my_y256 = 412 * 256 }
        if my_tilt != 0 {
            my_tilt += if my_tilt < 0 { 1 } else { -1 }
        }
        {
            let (my_code, l_offset, r_offset) = match my_tilt {
                -40..=-29 => ( 0, 23, 36),
                -28..=-22 => ( 1, 23, 36),
                -21..=-15 => ( 2, 22, 36),
                -14..=-8  => ( 3, 22, 37),
                -7..=-3   => ( 4, 21, 37),
                -2..=-1   => ( 5, 21, 37),
                0         => ( 6, 21, 37),
                1..=2     => ( 7, 21, 37),
                3..=7     => ( 8, 21, 37),
                8..=14    => ( 9, 21, 36),
                15..=21   => (10, 22, 36),
                22..=28   => (11, 22, 35),
                29..=40   => (12, 22, 35),
                _         => ( 6, 21, 37)
            };
            let my_pos = SpPos::new(my_x256 / 256, my_y256 / 256);
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
            if input_role_state.get(InputRole::Button4).0 {
                let (dx, dy) = {
                //    let t = (t_count % 16) - 8;
                //    if t < 0 { -t * 128 } else { t * 128 }
                    ((t_count % 8) * 200 ,-14 * 256 + 100)
                };
                let new_shot = Some(((my_x256 + 21 * 256, my_y256 + 10 * 256), (-dx, dy), 15));
                if let Some(i) = unused.pop() {
                    shots[i] = new_shot;
                } else {
                    shots.push(new_shot);
                }
                let new_shot = Some(((my_x256 + 35 * 256, my_y256 + 10 * 256), (dx, dy), 15));
                if let Some(i) = unused.pop() {
                    shots[i] = new_shot;
                } else {
                    shots.push(new_shot);
                }
                let new_shot = Some(((my_x256 + 28 * 256, my_y256 - 14 * 256), (0, dy), 15));
                if let Some(i) = unused.pop() {
                    shots[i] = new_shot;
                } else {
                    shots.push(new_shot);
                }
            }
            let mut sp_idx = 16;
            for i in 0..shots.len() {
                if let Some(((x256, y256), (dx, dy), c)) = shots[i] {
                    spr.sp(sp_idx).xy(x256 / 256, y256 /256).code(c).palette(1).visible(true);
                    let new_x256 = x256 + dx;
                    let new_y256 = y256 + dy;
                    if new_y256 < -32 * 256 {
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
        if input_role_state.get(InputRole::Button1).1 & 0b1111 == 0b1100 {
            display_info.rotation = display_info.rotation.turn_right();
            display_info.f_count = 0;
        }
        if wait_and_update::doing(&mut window, &mut spr, &mut bg, &mut display_info, &keyboard_map, &mut input_role_state) {
            break 'main_loop;
        }
        t_count += 1;
    }

sdl_context.mouse().show_cursor(true);
}
