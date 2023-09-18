pub mod bgsp_common;
mod bg_lib;
mod sp_lib;
pub mod bg_resources;
pub mod sp_texture_bank;
pub mod classic_sprite;
pub mod sp_resources;
pub mod bg_plane;

#[macro_export]
macro_rules! x {($e:expr) => { $e.0 }}
#[macro_export]
macro_rules! y {($e:expr) => { $e.1 }}

