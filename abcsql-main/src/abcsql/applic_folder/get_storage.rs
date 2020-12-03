#[cfg(feature = "sled-storage")]
use gluesql::{sled, SledStorage};

#[cfg(feature = "sled-storage")]
use std::convert::TryFrom;

use std::rc::Rc;

use crate::applic_folder::compileargs::*;

pub fn get_storage(how: Rc<HowToOpenStorage>) -> SledStorage {
    match &*how {
        HowToOpenStorage::NewStorage(filename) => {
            SledStorage::new(&filename).expect("get_storage() newdb problem")
        }
        HowToOpenStorage::OldStorage(filename) => {
            let config = sled::Config::default().path(&filename).temporary(false);
            SledStorage::try_from(config).expect("get_storage() olddb problem")
        }
    }
}
