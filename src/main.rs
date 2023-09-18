use shoot_proto::{
    bgsp_data::{
        bgchar_data,
        bgpal_data,
        spchar_data,
        sppal_data,
    },
    direction::*,
    input_role::*,

    bgsp_lib::{
        bgsp_common::*,
        bg_resources::*,
        bg_plane::*,
        classic_sprite::*,
        sp_resources::*,
        sp_texture_bank::*,
    },
};

use piston_window::*;
use sdl2_window::Sdl2Window;
use std::collections::BTreeMap;
// use once_cell::sync::OnceCell;

type GameWindow = piston_window::PistonWindow<sdl2_window::Sdl2Window>;

struct DisplayInfo {
    full_screen: bool,
    vm_rect_size: (i32, i32),
    rotation: Direction,
    pixel_scale: i32,
    margin: i32,
    f_count: i32,
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
            (piston_window::Key::Space, InputRole::Button7),
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

    bg.0.set_cur_pos(20, 20)
        .put_string("Test for shoot", Some(&CharAttributes::new(2, BgSymmetry::Normal)));
    spr.sp[0].code(6).palette(1).symmetry(SpSymmetry::Normal);
    spr.sp[1].code(13).palette(1).symmetry(SpSymmetry::Normal);
    spr.sp[2].code(14).palette(1).symmetry(SpSymmetry::Normal);
    spr.sp[3].code(15).palette(1).symmetry(SpSymmetry::Normal);
    spr.sp[4].code(16).palette(1).symmetry(SpSymmetry::Rotate270);

    'main_loop: loop {
        bg.0.set_cur_pos(25,0)
            .put_string(&format!("({:4}, {:4})", my_x256 / 256, my_y256 / 256), Some(&CharAttributes::new(1, BgSymmetry::Normal)))
            .put_achar(&AChar::new('*', if t_count % 4 < 2 { 0 } else { 1 }, BgSymmetry::Normal));
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
        if my_y256 > 416 * 256 { my_y256 = 416 * 256 }
        if my_tilt != 0 {
            my_tilt += if my_tilt < 0 { 1 } else { -1 }
        }
        let my_code = match my_tilt {
            -40..=-29 => 0,
            -28..=-22 => 1,
            -21..=-15 => 2,
            -14..=-8 => 3,
            -7..=-3 => 4,
            -2..=-1 => 5,
            0 => 6,
            1..=2 => 7,
            3..=7 => 8,
            8..=14 => 9,
            15..=21 => 10,
            22..=28 => 11,
            29..=40 => 12,
            _ => 6,
        };
        spr.sp[0].xy(my_x256 / 256, my_y256 / 256).code(my_code).visible(true);
        spr.sp[1].xy(my_x256 / 256, my_y256 / 256 - 16).visible(true);
        spr.sp[2].xy(my_x256 / 256 + 16, my_y256 / 256 - 16).visible(true);
        spr.sp[3].xy(my_x256 / 256, my_y256 / 256 - 32).visible(true);
        spr.sp[4].xy(my_x256 / 256 + 16, my_y256 / 256 - 32).visible(true);
        if input_role_state.get(InputRole::Button1).1 & 0b1111 == 0b1100 {
            display_info.rotation = display_info.rotation.turn_right();
            display_info.f_count = 0;
        }
        if wait_and_rendering(&mut window, &mut spr, &mut bg, &mut display_info, &keyboard_map, &mut input_role_state) {
            break 'main_loop;
        }
        t_count += 1;
    }

sdl_context.mouse().show_cursor(true);
}

fn wait_and_rendering(
    window: &mut GameWindow,
    spr: &mut SpResources,
    bg: &mut (BgPlane, BgPlane),
    info: &mut DisplayInfo,
    keyboard_map: &BTreeMap<piston_window::Key, Vec<InputRole>>,
    input_role_state: &mut InputRoleState,
) -> bool {
    let mut texture_context = window.create_texture_context();
    let texture_settings = TextureSettings::new();
    let _ = bg.0.rendering();
    let bg0_whole = Texture::from_image(
        &mut texture_context,
        bg.0.whole_image(),
        &texture_settings,
    ).unwrap();
    let _ = bg.1.rendering();
    let bg1_whole = Texture::from_image(
        &mut texture_context,
        bg.1.whole_image(),
        &texture_settings,
    ).unwrap();
    // Sprites
    let sp_drawn = Texture::from_image(
        &mut texture_context,
        &spr.rendering(info.vm_rect_size.0 as i32, info.vm_rect_size.1 as i32),
        &texture_settings,
    ).unwrap();

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(k)) = event.press_args() {
            if let Some(role_list) = keyboard_map.get(&k) {
                for role in role_list { input_role_state.set_true(*role) }
            }
        }
        if let Some(Button::Keyboard(k)) = event.release_args() {
            if let Some(role_list) = keyboard_map.get(&k) {
                for role in role_list { input_role_state.set_false(*role) }
            }
        }
        if let Event::Loop(Loop::Render(_)) = event {
            input_role_state.update_history();
            let window_size = window.size();
            window.draw_2d(&event, |context, graphics, _device| {
                let (vm_rect_width, vm_rect_height, pixel_scale, margin_2x) = (
                    info.vm_rect_size.0,
                    info.vm_rect_size.1,
                    info.pixel_scale,
                    info.margin * 2,
                );
                let (zoom, h_offset, v_offset) = {
                    let view_rect = {
                        let (width, height) = (vm_rect_width * pixel_scale, vm_rect_height * pixel_scale);
                        match info.rotation {
                            Direction::Up    | Direction::Down => (width, height),
                            Direction::Right | Direction::Left => (height, width),
                        }
                    };
                    let h_zoom = window_size.width / ((view_rect.0 + margin_2x) as f64);
                    let v_zoom = window_size.height / ((view_rect.1  + margin_2x) as f64);
                    let zoom = h_zoom.min(v_zoom);
                    let h_offset =  (window_size.width - (view_rect.0 as f64) * zoom) / 2.0;
                    let v_offset =  (window_size.height - (view_rect.1 as f64) * zoom) / 2.0;
                    (zoom, h_offset, v_offset)
                };
                let base_transform = context.transform.zoom(zoom).trans(h_offset / zoom, v_offset / zoom);
                let transform = match info.rotation {
                    Direction::Up    => base_transform.rot_deg(  0.0).trans(0.0, 0.0),
                    Direction::Right => base_transform.rot_deg( 90.0).trans(0.0, -((vm_rect_height * pixel_scale) as f64)),
                    Direction::Down  => base_transform.rot_deg(180.0).trans(-((vm_rect_width * pixel_scale) as f64), -((vm_rect_height * pixel_scale) as f64)),
                    Direction::Left  => base_transform.rot_deg(270.0).trans(-((vm_rect_width * pixel_scale) as f64), 0.0),
                };
                let draw_inside = draw_state::DrawState::new_inside();
                if info.f_count < 4 {
                    // Initialize
                    graphics.clear_color([0.0, 0.0, 0.0, 1.0]);
                    graphics.clear_stencil(0);
                    rectangle::Rectangle::new([1.0; 4]).draw(
                        [0.0, 0.0, (vm_rect_width * pixel_scale) as f64, (vm_rect_height * pixel_scale) as f64],
                        &draw_state::DrawState::new_clip(),
                        transform,
                        graphics,
                    );
                } else {
                    // Clear
                    rectangle::Rectangle::new([0.0, 0.0, 0.0, 1.0]).draw(
                        [0.0, 0.0, (vm_rect_width * pixel_scale) as f64, (vm_rect_height * pixel_scale) as f64],
                        &draw_inside,
                        transform,
                        graphics,
                    );
                }
                // BG1
                image::draw_many(
                    bg.1.draw_rects(), [1.0, 1.0, 1.0, 1.0],
                    &bg1_whole,
                    &draw_inside,
                    transform,
                    graphics,
                );
                // Sprites
                image::Image::new().draw(
                    &sp_drawn,
                    &draw_inside,
                    transform,
                    graphics,
                );
                // BG0
                image::draw_many(
                    bg.0.draw_rects(), [1.0, 1.0, 1.0, 1.0],
                    &bg0_whole,
                    &draw_inside,
                    transform,
                    graphics,
                );
            });
            info.f_count += 1;
            return false;
        }
    }
    true
}
