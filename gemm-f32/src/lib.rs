#![cfg_attr(feature = "nightly", feature(stdsimd), feature(avx512_target_feature))]

pub mod gemm;
pub mod microkernel;

#[macro_use]
extern crate gemm_common;
