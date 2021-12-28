#![forbid(unsafe_code)]
#![warn(
    // restriction lints
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::exit,
    clippy::filetype_is_file,
    clippy::fn_to_numeric_cast_any,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::lossy_float_literal,
    clippy::multiple_inherent_impl,
    clippy::same_name_method,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::unneeded_field_pattern,
    clippy::str_to_string,
    clippy::string_to_string,
    clippy::undocumented_unsafe_blocks,
    clippy::unnecessary_self_imports,
    clippy::unseparated_literal_suffix,

    // nursery lints
    clippy::debug_assert_with_mut_call,
    clippy::equatable_if_let,
    clippy::fallible_impl_from,
    clippy::option_if_let_else,
    clippy::path_buf_push_overwrite,
    clippy::use_self,

    // pedantic lints
    clippy::cloned_instead_of_copied,
    clippy::filter_map_next,
    clippy::if_not_else,
    clippy::from_iter_instead_of_collect,
    clippy::implicit_clone,
    clippy::inefficient_to_string,
    clippy::large_digit_groups,
    clippy::large_stack_arrays,
    clippy::large_types_passed_by_value,
    clippy::let_unit_value,
    clippy::macro_use_imports,
    clippy::manual_assert,
    clippy::manual_ok_or,
    clippy::map_unwrap_or,
    clippy::match_bool,
    clippy::mut_mut,
    clippy::needless_bitwise_bool,
    clippy::needless_pass_by_value,
    clippy::option_option,
    clippy::ptr_as_ptr,
    clippy::redundant_closure_for_method_calls,
    clippy::semicolon_if_nothing_returned,
    clippy::string_add_assign,
    clippy::trait_duplication_in_bounds,
    clippy::type_repetition_in_bounds,
    clippy::unicode_not_nfc,
    clippy::unnested_or_patterns,
    clippy::unreadable_literal,
    clippy::unused_self,
    clippy::unused_async,
    clippy::used_underscore_binding,
    clippy::verbose_bit_mask,
)]

use macroquad::prelude::*;
use macroquad::ui::{*, widgets::*};
use egui_macroquad::egui;
use enumflags2::{bitflags, BitFlags};
use std::collections::HashMap;
use std::path::PathBuf;
//use tinyfiledialogs::{open_file_dialog, color_chooser_dialog, DefaultColorValue};
//use include_dir::{include_dir, Dir};

const EXAMPLE_TOML_OUTPUT: &str = r#"
[[entities]]
name = "pinik"
x = 0
y = 0
w = 26
h = 26

[[entities]]
name = "bad_guy"
x = 200
y = 200
w = 120
h = 140
"#;

type Map<K, V> = HashMap<K, V>;

#[bitflags]
#[repr(u8)]
#[derive(PartialEq, Copy, Clone, Debug)]
enum Settings {
    Forget, // Delete old settings of the app and don't save it as long as this option is set
    Reload, // Reload all the previous images and place them like the last time
}

#[macroquad::main("Jubilant")]
async fn main() {
    let settings: BitFlags<Settings> = BitFlags::empty(); // TODO: Load from settings.toml

    let mut text_output = EXAMPLE_TOML_OUTPUT.to_owned();

    let textures: Map<String, Texture2D> = Map::new();

    loop {
        clear_background(WHITE);

        // Canvas
        for texture in textures.values().copied() {
            draw_texture(texture, 0.0, 0.0, WHITE);
        }

        // GUI
        egui_macroquad::ui(|ctx| {
            // Output Window
            egui::Window::new("Output")
                .min_width(200_f32)
                .vscroll(true)
                .show(ctx, |ui| {
                    if ui.button("Copy").clicked() {
                        todo!();
                    }
                    if ui.button("Generate").clicked() {
                        todo!();
                    }
                    ui.text_edit_multiline(&mut text_output.as_str());
                });
            // Input Window
            egui::Window::new("Input")
                .min_width(200_f32)
                .vscroll(true)
                .show(ctx, |ui| {

                });
        });

        egui_macroquad::draw();

        next_frame().await;
    }
}
