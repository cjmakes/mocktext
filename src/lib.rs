#![cfg(any(target_arch = "x86", target_arch = "x86_64"))]

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[inline(always)]
pub fn round_up<const M: usize>(n: usize) -> usize {
    (n + (M - 1)) / M * M
}

fn mock_sse_mut<'a>(text: &'a mut [u8]) -> &'a [u8] {
    unsafe {
        let tp = text.as_ptr();

        for i in (0..text.len()).step_by(16) {
            let txt = _mm_loadu_si128(tp.add(i) as *const __m128i);
            let cmp = _mm_cmpeq_epi8(txt, _mm_set1_epi8(32));
            #[allow(overflowing_literals)]
            let alt = _mm_unpacklo_epi8(_mm_set1_epi8(0), _mm_set1_epi8(0xff));
            let msk = _mm_or_si128(cmp, alt);
            #[allow(overflowing_literals)]
            let inv = _mm_xor_si128(msk, _mm_set1_epi8(0xff));
            let dif = _mm_and_si128(inv, _mm_set1_epi8(32));
            let new = _mm_sub_epi8(txt, dif);
            _mm_storeu_si128(tp.add(i) as *mut __m128i, new);
        }
    }
    text
}

fn mock_sse(text: &str) -> String {
    let len = round_up::<16>(text.len());

    // TODO extend original string instead of allocating a new one
    let mut txt = vec![b' '; len];
    txt[..text.len()].clone_from_slice(text.as_bytes());

    let txt = mock_sse_mut(&mut txt);

    std::str::from_utf8(&txt).unwrap().trim().to_string()
}

fn mock_fallback(text: &str) -> String {
    let bytes: Vec<u8> = text
        .to_string()
        .chars()
        .zip(0..)
        .map(|(t, m)| {
            if t == ' ' || m % 2 != 0 {
                t as u8
            } else {
                t as u8 - 32
            }
        })
        .clone()
        .collect();

    std::str::from_utf8(&bytes).unwrap().trim().to_string()
}

pub fn mock(text: &str) -> String {
    if is_x86_feature_detected!("sse3") {
        return mock_sse(text);
    }
    mock_fallback(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        assert_eq!(mock("abcdefghijklmnop"), "AbCdEfGhIjKlMnOp");
    }
    #[test]
    fn basic_space() {
        assert_eq!(mock("abc efg ijkl nop"), "AbC EfG IjKl nOp");
    }
    #[test]
    fn basic_len_gt_16() {
        assert_eq!(
            mock("abcdefghijklmnopqrstuvwxyz"),
            "AbCdEfGhIjKlMnOpQrStUvWxYz"
        );
    }
}
