#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_q31() {
        let vec_a: [i32; 3] = [1, 2, 3];
        let vec_b: [i32; 3] = [4, 5, 6];
        let mut dest: [i32; 3] = Default::default();

        unsafe {
            arm_add_q31(vec_a.as_ptr(), vec_b.as_ptr(), dest.as_mut_ptr(), 3);
        }

        assert!(dest == [5, 7, 9])
    }
}
