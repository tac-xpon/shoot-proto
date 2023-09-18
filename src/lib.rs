pub mod bgsp_lib {
    pub mod bgsp_common;
    pub mod bg_lib;
    pub mod bg_resources;
    pub mod bg_plane;
    pub mod sp_lib;
    pub mod classic_sprite;
    pub mod sp_resources;
    pub mod sp_texture_bank;
}

pub mod bgsp_data {
    pub mod bgchar_data;
    pub mod bgpal_data;
    pub mod spchar_data;
    pub mod sppal_data;
}

pub mod direction;
pub mod input_role;

#[macro_export]
macro_rules! x {($e:expr) => { $e.0 }}
#[macro_export]
macro_rules! y {($e:expr) => { $e.1 }}
