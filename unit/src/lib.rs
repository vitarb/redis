mod dict;
mod sds;

#[cfg(test)]
mod tests {
    use std::ffi::{c_char, c_void, CStr};
    use crate::dict::{dictAdd, dictCreate, dictFind, dictType};
    use crate::sds::{sds, sdsnew};
    use super::*;

    extern "C" {
        static dbDictType: dictType;
    }

    #[test]
    fn it_works() {
        let dt = &mut unsafe { dbDictType };
        let dict = unsafe { dictCreate(dt) };
        let key_ptr = "hello\0".as_ptr() as *const c_char;
        let val_ptr = "world\0".as_ptr() as *const c_char;

        let key = unsafe { &mut sdsnew(key_ptr) as *mut sds as *mut c_void };
        let val = unsafe { &mut sdsnew(val_ptr) as *mut sds as *mut c_void };

        unsafe {
            dictAdd(dict, key, val);
            let de = dictFind(dict, key);
        }
    }
}
