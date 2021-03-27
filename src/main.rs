#![cfg(any(target_arch = "x86", target_arch = "x86_64"))]

use std::io;

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[inline(always)]
pub fn round_up<const M: usize>(n: usize) -> usize {
    (n + (M - 1)) / M * M
}

pub fn mock_sse(text: &str) -> String {
    let len = round_up::<16>(text.len());

    // TODO extend original string instead of allocating a new one
    let mut txt = vec![b' '; len];
    txt[..text.len()].clone_from_slice(text.as_bytes());

    unsafe {
        let tp = txt.as_ptr();

        for i in (0..len).step_by(16) {
            // Load a chunk of text
            let txt = _mm_loadu_si128(tp.add(i) as *const __m128i);
            // Preserve spaces
            let cmp = _mm_cmpeq_epi8(txt, _mm_set1_epi8(32));
            // Make alternating array [ff,0,ff...]
            #[allow(overflowing_literals)]
            let alt = _mm_unpacklo_epi8(_mm_set1_epi8(0), _mm_set1_epi8(0xff));
            // Combine alternating and spaces
            let msk = _mm_or_si128(cmp, alt);
            // Invert mask
            #[allow(overflowing_literals)]
            let inv = _mm_xor_si128(msk, _mm_set1_epi8(0xff));
            // Convert mask to case offset, a-A = 32
            let dif = _mm_and_si128(inv, _mm_set1_epi8(32));
            // Upcase with mask
            let new = _mm_sub_epi8(txt, dif);
            // Store result
            _mm_storeu_si128(tp.add(i) as *mut __m128i, new);
        }
    }

    std::str::from_utf8(&txt).unwrap().trim().to_string()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read");
    println!("{}", &mock_sse(&input.trim_matches('\u{000a}').to_string()));
}
