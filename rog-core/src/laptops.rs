use rog_client::{
    aura_modes::{
        AuraModes, BREATHING, COMET, FLASH, HIGHLIGHT, LASER, MULTISTATIC, PULSE, RAIN, RAINBOW,
        RGB, RIPPLE, SINGLE, STAR, STROBE,
    },
};
use log::{info, warn};

static HELP_ADDRESS: &str = "https://github.com/flukejones/rog-core";

pub(crate) fn match_laptop() -> LaptopBase {
    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();
        if device_desc.vendor_id() == 0x0b05 {
            match device_desc.product_id() {
                0x1869 | 0x1866 => return select_1866_device(device_desc.product_id()),
                0x1854 => {
                    info!("Found GL753 or similar");
                    return LaptopBase {
                        usb_vendor: 0x0B05,
                        usb_product: 0x1854,
                        //from `lsusb -vd 0b05:1866`
                        led_endpoint: 0x04,
                        //from `lsusb -vd 0b05:1866`
                        key_endpoint: 0x83,
                        supported_modes: vec![SINGLE, BREATHING, STROBE],
                        support_animatrix: false,
                    };
                }
                _ => {}
            }
        }
    }
    panic!("could not match laptop");
}

fn select_1866_device(prod: u16) -> LaptopBase {
    let dmi = sysfs_class::DmiId::default();
    let board_name = dmi.board_name().expect("Could not get board_name");
    let prod_name = dmi.product_name().expect("Could not get board_name");

    info!("Product name: {}", prod_name.trim());
    info!("Board name: {}", board_name.trim());

    let mut laptop = LaptopBase {
        usb_vendor: 0x0B05,
        usb_product: prod,
        //from `lsusb -vd 0b05:1866`
        led_endpoint: 0x04,
        //from `lsusb -vd 0b05:1866`
        key_endpoint: 0x83,
        supported_modes: vec![],
        support_animatrix: false,
    };

    // AniMe, no RGB
    if board_name.starts_with("GA401") {
        info!("No RGB control available");
        // TODO: actual check for the AniMe device here
        laptop.support_animatrix = true;
    // No AniMe, no RGB
    } else if board_name.starts_with("GA502") || board_name.starts_with("GU502") {
        info!("No RGB control available");
    // RGB, per-key settings, no zones
    } else if board_name.starts_with("GX502")
        || board_name.starts_with("GX701")
        || board_name.starts_with("G531")
        || board_name.starts_with("GL531")
        || board_name.starts_with("G532")
    {
        laptop.supported_modes = vec![
            SINGLE, BREATHING, STROBE, RAINBOW, STAR, RAIN, HIGHLIGHT, LASER, RIPPLE, PULSE, COMET,
            FLASH, RGB,
        ];
    } else if board_name.starts_with("G531")
        || board_name.starts_with("G731")
    {
        laptop.supported_modes = vec![
            SINGLE, BREATHING, STROBE, RAINBOW, STAR, RAIN, HIGHLIGHT, LASER, RIPPLE, PULSE, COMET,
            FLASH, MULTISTATIC, RGB,
        ];
    // RGB, limited effects, no zones
    } else if board_name.starts_with("G512LI") || board_name.starts_with("G712LI") {
        laptop.supported_modes = vec![SINGLE, BREATHING, STROBE, RAINBOW, PULSE];
    // RGB, limited effects, 4-zone RGB
    } else if board_name.starts_with("GM501")
        || board_name.starts_with("GX531")
        || board_name.starts_with("G512")
        || board_name.starts_with("G712")
    {
        laptop.supported_modes = vec![SINGLE, BREATHING, STROBE, RAINBOW, PULSE, MULTISTATIC];
    } else {
        warn!(
            "Unsupported laptop, please request support at {}",
            HELP_ADDRESS
        );
        warn!("Continuing with minimal support")
    }

    if !laptop.supported_modes.is_empty() {
        info!("Supported Keyboard LED modes are:");
        for mode in &laptop.supported_modes {
            let mode = <&str>::from(&<AuraModes>::from(*mode));
            info!("- {}", mode);
        }
        info!(
            "If these modes are incorrect or missing please request support at {}",
            HELP_ADDRESS
        );
    }

    laptop
}

pub(super) struct LaptopBase {
    usb_vendor: u16,
    usb_product: u16,
    led_endpoint: u8,
    key_endpoint: u8,
    supported_modes: Vec<u8>,
    support_animatrix: bool,
}

impl LaptopBase {
    pub(super) fn led_endpoint(&self) -> u8 {
        self.led_endpoint
    }
    pub(super) fn key_endpoint(&self) -> u8 {
        self.key_endpoint
    }
    pub(super) fn usb_vendor(&self) -> u16 {
        self.usb_vendor
    }
    pub(super) fn usb_product(&self) -> u16 {
        self.usb_product
    }
    pub(super) fn supported_modes(&self) -> &[u8] {
        &self.supported_modes
    }
    pub(super) fn support_animatrix(&self) -> bool {
        self.support_animatrix
    }
    pub(super) fn set_support_animatrix(&mut self, supported: bool) {
        self.support_animatrix = supported;
    }
}
