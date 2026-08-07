use core::{
    cell::RefCell,
    ops::{Shl, Shr},
};

use alloc::{boxed::Box, rc::Rc};
use pebble_rust_2026::{
    APP, DictionaryView, GColor, InboxSize, hex_color, log_c_str, message_keys,
};

message_keys!(keys);

pub struct Config {
    pub background_color: GColor,
    pub digit_colors: (GColor, GColor, GColor, GColor),
}

fn color_from_rgb(r: u8, g: u8, b: u8) -> GColor {
    GColor {
        argb: 0b11000000u8 | r.shl(4) | g.shl(2) | b,
    }
}

fn color_from_hex(v: u32) -> GColor {
    color_from_rgb(
        (v.shr(22) & 0xffu32) as u8,
        (v.shr(14) & 0xffu32) as u8,
        (v.shr(6) & 0xffu32) as u8,
    )
}

fn get_saved_color(key: &'static u32, default: GColor) -> GColor {
    if let Some(c) = APP.persist.read_int(key) {
        color_from_hex(c as u32)
    } else {
        default
    }
}

fn get_message_color(message: &DictionaryView, key: &'static u32) -> Option<GColor> {
    if let Some(v) = message.get(key)
        && let Some(i) = v.as_u32()
    {
        APP.persist.write_int(key, i as i32);
        return Some(color_from_hex(i));
    }
    None
}

pub fn load_config(mut callback: Box<dyn FnMut(&Config)>) -> Rc<RefCell<Config>> {
    let config = Config {
        background_color: get_saved_color(keys::CONFIG_BACKGROUND_COLOR, hex_color!("#000")),
        digit_colors: (
            get_saved_color(keys::CONFIG_DIGIT_0_COLOR, hex_color!("#fff")),
            get_saved_color(keys::CONFIG_DIGIT_1_COLOR, hex_color!("#fff")),
            get_saved_color(keys::CONFIG_DIGIT_2_COLOR, hex_color!("#fff")),
            get_saved_color(keys::CONFIG_DIGIT_3_COLOR, hex_color!("#fff")),
        ),
    };

    callback(&config);

    let config = Rc::new(RefCell::new(config));

    APP.set_message_handler({
        let config = config.clone();
        move |message| {
            log_c_str(c"got message");
            let mut config = config.borrow_mut();
            if let Some(v) = get_message_color(message, keys::CONFIG_BACKGROUND_COLOR) {
                config.background_color = v;
            }
            if let Some(v) = get_message_color(message, keys::CONFIG_DIGIT_0_COLOR) {
                config.digit_colors.0 = v;
            }

            if let Some(v) = get_message_color(message, keys::CONFIG_DIGIT_1_COLOR) {
                config.digit_colors.1 = v;
            }

            if let Some(v) = get_message_color(message, keys::CONFIG_DIGIT_2_COLOR) {
                config.digit_colors.2 = v;
            }

            if let Some(v) = get_message_color(message, keys::CONFIG_DIGIT_3_COLOR) {
                config.digit_colors.3 = v;
            }

            callback(&config)
        }
    });

    APP.open_inbox(InboxSize::Exact {
        inbox: 150,
        outbox: 150,
    })
    .unwrap();

    config
}
