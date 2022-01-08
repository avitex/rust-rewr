use std::unimplemented;

use dangerous::input::{Bytes, Pattern};

pub struct OneOf<T, const N: usize>(pub [T; N]);

unsafe impl<'i, const N: usize> Pattern<Bytes<'i>> for OneOf<u8, N> {
    fn find_match(self, input: &Bytes<'i>) -> Option<(usize, usize)> {
        let n = self.0;
        let haystack = input.as_dangerous();
        #[rustfmt::skip]
        let result = match N {
            0 => None,
            1 => memchr::memchr(n[0], haystack),
            2 => memchr::memchr2(n[0], n[1], haystack),
            3 => memchr::memchr3(n[0], n[1], n[3], haystack),
            4 => jetscii::bytes!(n[0], n[1], n[2], n[3]).find(haystack),
            5 => jetscii::bytes!(n[0], n[1], n[2], n[3], n[4]).find(haystack),
            6 => jetscii::bytes!(n[0], n[1], n[2], n[3], n[4], n[5]).find(haystack),
            7 => jetscii::bytes!(n[0], n[1], n[2], n[3], n[4], n[5], n[6]).find(haystack),
            8 => jetscii::bytes!(n[0], n[1], n[2], n[3], n[4], n[5], n[6], n[7]).find(haystack),
            9 => jetscii::bytes!(n[0], n[1], n[2], n[3], n[4], n[5], n[6], n[7], n[8]).find(haystack),
            10 => jetscii::bytes!(n[0], n[1], n[2], n[3], n[4], n[5], n[6], n[7], n[8], n[9]).find(haystack),
            11 => jetscii::bytes!(n[0], n[1], n[2], n[3], n[4], n[5], n[6], n[7], n[8], n[9], n[10]).find(haystack),
            12 => jetscii::bytes!(n[0], n[1], n[2], n[3], n[4], n[5], n[6], n[7], n[8], n[9], n[10], n[11]).find(haystack),
            13 => jetscii::bytes!(n[0], n[1], n[2], n[3], n[4], n[5], n[6], n[7], n[8], n[9], n[10], n[11], n[12]).find(haystack),
            14 => jetscii::bytes!(n[0], n[1], n[2], n[3], n[4], n[5], n[6], n[7], n[8], n[9], n[10], n[11], n[12], n[13]).find(haystack),
            15 => jetscii::bytes!(n[0], n[1], n[2], n[3], n[4], n[5], n[6], n[7], n[8], n[9], n[10], n[11], n[12], n[13], n[14]).find(haystack),
            16 => jetscii::bytes!(n[0], n[1], n[2], n[3], n[4], n[5], n[6], n[7], n[8], n[9], n[10], n[11], n[12], n[13], n[14], n[16]).find(haystack),
            _ => unimplemented!(),
        };
        result.map(|index| (index, 1))
    }

    fn find_reject(self, _input: &Bytes<'i>) -> Option<usize> {
        unimplemented!()
    }
}

// unsafe impl<'i> Pattern<String<'i>> for OneOf2<char> {
//     fn find_match(self, input: &String<'i>) -> Option<(usize, usize)> {
//         if self.0.is_ascii() && self.1.is_ascii() {
//             OneOf2(self.0 as u8, self.1 as u8).find_match(&input.clone().into_bytes())
//         } else {
//             unimplemented!()
//         }
//     }

//     fn find_reject(self, input: &String<'i>) -> Option<usize> {
//         unimplemented!()
//     }
// }
