use crate::constants::DisplayMode;

pub struct King;

impl King {
    pub fn to_string(display_mode: &DisplayMode) -> &'static str {
        match display_mode {
            DisplayMode::Default => {
                "\
               ✚\n\
             ▞▀▄▀▚\n\
             ▙▄█▄▟\n\
              ████▌\n\
            ▗█████▖\n\
                \n\
                "
            }
            DisplayMode::ASCII => "K",
        }
    }
}
