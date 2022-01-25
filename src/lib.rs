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
