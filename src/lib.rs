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
    pub fn new(from1: &String, from2: &String) -> Self {
        let mut container = Vec::with_capacity(from1.len() + from2.len());

        let len1 = unsafe {
            container.extend_from_slice(std::mem::transmute(from1.as_str()));
             container.len()
        };

        let len2 = unsafe {
            container.extend_from_slice(std::mem::transmute(from2.as_str()));
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

#[cfg(test)]
mod tests {
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
}
