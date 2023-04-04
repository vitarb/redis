mod dict;
mod sds;

#[cfg(test)]
mod tests {
    use std::ffi::{c_char, c_void, CStr};
    use std::ptr::{null_mut};
    use crate::dict::{dictAdd, dictCreate, dictFind, dictType};
    use crate::sds::{sds, sdsnew};
    use super::*;

    extern "C" {
        static dbDictType: dictType;
    }

    #[test]
    fn it_works() {
        let dict = unsafe { dictCreate(&mut unsafe { dbDictType }) };
        let key = sds("hello\0");
        let val = sds("world\0");
        let de = unsafe {
            dictAdd(dict, key, val);
            dictFind(dict, key)
        };
        assert_ne!(de, null_mut());
        assert_eq!(unsafe {CStr::from_ptr((*de).key as *const c_char) }.to_str(), Ok("hello"));
        assert_eq!(unsafe {CStr::from_ptr((*de).v.val as *const c_char) }.to_str(), Ok("world"));
    }

    fn sds(s: &str) -> *mut c_void {
        let key_ptr = s.as_ptr() as *const c_char;
        let key = unsafe { sdsnew(key_ptr) as *mut c_void };
        key
    }
}
