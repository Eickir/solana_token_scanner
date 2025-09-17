use crate::platforms::constants::PUMPFUN_PROGRAM_ID;
use crate::platforms::platforms::Platform;

pub fn identify_platform(accounts: &Vec<String>) -> Option<Platform> {

    if accounts.contains(&PUMPFUN_PROGRAM_ID.to_string()) {
        Some(Platform::PumpFun)
    } else {
        None
    }

}