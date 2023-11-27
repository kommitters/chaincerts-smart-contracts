// Base32 Encoding (lowercase) written in pure Rust. No std, no alloc, no crates.
// Ported from https://github.com/golang/go/blob/26b5783b72376acd0386f78295e678b9a6bff30e/src/encoding/base32/base32.go#L111-L184
//
// Modifications:
//    * Removed logic supporting padding.
//    * Hardcoded the Base32 alphabet in lowercase.
//    * Use a fixed length pre-allocated destination.
//    * Ported to Rust.
//
// Original Copyright notice:
//
// Copyright (c) 2009 The Go Authors. All rights reserved.
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//    * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//    * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//    * Neither the name of Google Inc. nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyz234567";

pub fn encode(dst: &mut [u8], src: &[u8]) {
    let mut src_idx = 0;
    let mut dst_idx = 0;

    while src_idx < src.len() {
        let mut b = [0u8; 8];

        let src_size = 5.min(src.len() - src_idx);
        for i in (1..src_size + 1).rev() {
            match i {
                5 => {
                    b[7] = src[src_idx + 4] & 0x1f;
                    b[6] = src[src_idx + 4] >> 5;
                }
                4 => {
                    b[6] |= (src[src_idx + 3] << 3) & 0x1f;
                    b[5] = (src[src_idx + 3] >> 2) & 0x1f;
                    b[4] = src[src_idx + 3] >> 7;
                }
                3 => {
                    b[4] |= (src[src_idx + 2] << 1) & 0x1f;
                    b[3] = (src[src_idx + 2] >> 4) & 0x1f;
                }
                2 => {
                    b[3] |= (src[src_idx + 1] << 4) & 0x1f;
                    b[2] = (src[src_idx + 1] >> 1) & 0x1f;
                    b[1] = (src[src_idx + 1] >> 6) & 0x1f;
                }
                _ => {
                    b[1] |= (src[src_idx] << 2) & 0x1f;
                    b[0] = src[src_idx] >> 3;
                }
            }
        }

        let dst_size = (8).min(dst.len() - dst_idx);
        for i in 0..dst_size {
            dst[dst_idx] = ALPHABET[(b[i] as usize) & 31];
            dst_idx += 1;
        }

        src_idx += 5;
    }
}
