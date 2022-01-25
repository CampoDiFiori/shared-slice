///
/// yeah maybe we should
struct OwningContainer {
    container: Vec<u8>,
    reference1: *const u8,
    len1: usize,
    reference2: *const u8,
    len2: usize,
    _pin: std::marker::PhantomPinned,
}

impl OwningContainer {
    pub fn new(from1: &str, from2: &str) -> Self {
        let mut container = Vec::with_capacity(from1.len() + from2.len());

        let len1 = unsafe {
            container.extend_from_slice(std::mem::transmute(from1));
            container.len()
        };

        let len2 = unsafe {
            container.extend_from_slice(std::mem::transmute(from2));
            container.len() - len1
        };

        let mut ret = Self {
            container,
            reference1: std::ptr::null(),
            len1,
            reference2: std::ptr::null(),
            len2,
            _pin: std::marker::PhantomPinned,
        };

        //        let ret_reference: *mut String = &mut ret.container;

        ret.reference1 = ret.container.as_ptr();

        unsafe {
            ret.reference2 = ret.container.as_ptr().offset(ret.len1 as isize);
        }

        ret
    }

    pub fn reference1(&self) -> &str {
        unsafe {
            let slice = std::slice::from_raw_parts(self.reference1, self.len1);
            let reference: &str = std::str::from_utf8_unchecked(slice);
            reference
        }
    }

    pub fn reference2(&self) -> &str {
        unsafe {
            let slice = std::slice::from_raw_parts(self.reference2, self.len2);
            let reference: &str = std::str::from_utf8_unchecked(slice);
            reference
        }
    }
}

use derive_multislice::MultiSlice;

#[derive(MultiSlice)]
struct SomeStruct<'a> {
    f1: &'a [u8],
    f2: &'a [u32],
}

#[derive(MultiSlice)]
struct SomeOtherStruct<'a> {
    f1: &'a [u32],
}

#[cfg(test)]
mod tests {
    use derive_multislice::MultiSlice;

    use crate::SomeStruct;
    use crate::SomeOtherStruct;

    use super::OwningContainer;

    #[test]
    fn it_works() {
        let from1 = String::from("Maybe?");
        let from2 = String::from("Yyy?");
        let o1 = OwningContainer::new(&from1, &from2);
        let o2 = o1;
        assert_eq!(from1.as_str(), o2.reference1());
        assert_eq!(from2.as_str(), o2.reference2());
    }

    #[test]
    fn transmute() {
        let f1: &[u32] = &[257, 2, 3, 4];

        let (_, f1_ptr, _) = unsafe { f1.align_to::<u8>() };
        let p = f1_ptr.as_ptr() as *const std::ffi::c_void;
        let p = p as *const u32;

        let f1_end: &[u32] = unsafe { std::slice::from_raw_parts(p, 4) };

        assert_eq!(f1_ptr.len(), 4 * 4);
        assert_eq!(f1_end, f1);
    }

    #[test]
    fn proc_macro_simple() {
        let f1 = [1, 2, 3, 4];

        let ss = SomeOtherStruct::new(&f1);
        let f1_1 = ss.f1();

        assert_eq!(f1, f1_1);
    }

    #[test]
    fn proc_macro_new() {
        let f1 = &[1, 2, 3, 4];
        let f2 = &[1];

        let ss = SomeStruct::new(f1, f2);
        let f1_1 = ss.f1();
        let f2_1 = ss.f2();

        assert_eq!(f1, f1_1);
        assert_eq!(f2, f2_1);
    }

    #[derive(MultiSlice)]
    struct Unaligned<'a> {
        f1: &'a [i32],
        f2: &'a [u8],
    }

    #[test]
    fn proc_macro_unaligned() {
        let f1 = &[1, -25, 257];
        let f2 = &[1, 0, 255, 4, 5];

        let a = Unaligned::new(f1, f2);

        // assert_eq!(a.container(), &[1, 2]);

        let f1_1 = a.f1();
        let f2_1 = a.f2();

        assert_eq!(f1, f1_1);
        assert_eq!(f2, f2_1);
    }
}
