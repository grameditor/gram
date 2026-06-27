use objc2::{msg_send, runtime::AnyObject};
use objc2_app_kit::{
    NSAppearanceName, NSAppearanceNameAqua, NSAppearanceNameDarkAqua, NSAppearanceNameVibrantDark,
    NSAppearanceNameVibrantLight,
};

use crate::WindowAppearance;

impl WindowAppearance {
    pub(crate) unsafe fn from_native(appearance: *mut AnyObject) -> Self {
        unsafe {
            let name: &NSAppearanceName = msg_send![appearance, name];
            if name == NSAppearanceNameVibrantLight {
                Self::VibrantLight
            } else if name == NSAppearanceNameVibrantDark {
                Self::VibrantDark
            } else if name == NSAppearanceNameAqua {
                Self::Light
            } else if name == NSAppearanceNameDarkAqua {
                Self::Dark
            } else {
                println!("unknown appearance");
                Self::Light
            }
        }
    }
}
