use std::io::Read;

use hidapi::{HidApi, HidDevice};

use crate::firmware::Firmware;
use crate::Result;

pub struct FlashingOptions {
    pub offset: u32,
}

// impl FlashingOptions {
//     /// This function should be thereaded to not block the UI and send progress updates
//     pub fn flash<T: Read>(firmware: Firmware<T>, device: HidDevice) -> Result<()> {

//         // device.send_feature_report();
//         Ok(())
//     }
// }
