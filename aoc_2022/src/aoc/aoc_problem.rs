use std::fmt::{Display, Formatter, Result};
use Solution::*;

pub trait AoCProblem {
    /// Sets up an `AoCProblem` structure. This will parse the `input` vector so that it can be
    /// used for both parts of the problem. The parsed inputs should be made available as data
    /// members in the structure for easy access.
    ///
    /// # Parameters
    /// - `input`: The input.
    ///
    /// # Returns
    /// - The `AoCProblem`.
    fn prepare(input: &str) -> Self
    where
        Self: Sized;

    /// Solves part 1 of the day's Advent of Code problem.
    ///
    /// # Returns
    /// - The solution to part 1.
    fn part1(&mut self) -> Solution;

    /// Solves part 2 of the day's Advent of Code problem.
    ///
    /// # Returns
    /// - The solution to part 2.
    fn part2(&mut self) -> Solution;
}

#[derive(Clone)]
pub enum Solution {
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    I128(i128),
    U128(u128),
    ISize(isize),
    USize(usize),
    Str(String),
    Char(char),
}

// Everything below here was taken from
// https://github.com/agubelu/AoC-rust-template
// and modified to work with my template.
//
//
// Credit goes to https://github.com/agubelu for the
// code.

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            I8(x) => x.fmt(f),
            I16(x) => x.fmt(f),
            I32(x) => x.fmt(f),
            I64(x) => x.fmt(f),
            I128(x) => x.fmt(f),
            ISize(x) => x.fmt(f),
            U8(x) => x.fmt(f),
            U16(x) => x.fmt(f),
            U32(x) => x.fmt(f),
            U64(x) => x.fmt(f),
            U128(x) => x.fmt(f),
            USize(x) => x.fmt(f),
            Str(x) => x.fmt(f),
            Char(x) => x.fmt(f),
        }
    }
}

macro_rules! impl_from {
    ($type:ident, $kind:ident) => {
        impl From<$type> for Solution {
            fn from(sol: $type) -> Self {
                Self::$kind(sol)
            }
        }
    };
}

impl_from!(i8, I8);
impl_from!(i16, I16);
impl_from!(i32, I32);
impl_from!(i64, I64);
impl_from!(i128, I128);
impl_from!(isize, ISize);
impl_from!(u8, U8);
impl_from!(u16, U16);
impl_from!(u32, U32);
impl_from!(u64, U64);
impl_from!(u128, U128);
impl_from!(usize, USize);
impl_from!(String, Str);
impl_from!(char, Char);

macro_rules! impl_from_ref {
    ($type:ident, $kind:ident) => {
        impl From<&$type> for Solution {
            fn from(sol: &$type) -> Self {
                Self::$kind(sol.clone())
            }
        }
    };
}

impl_from_ref!(i8, I8);
impl_from_ref!(i16, I16);
impl_from_ref!(i32, I32);
impl_from_ref!(i64, I64);
impl_from_ref!(i128, I128);
impl_from_ref!(isize, ISize);
impl_from_ref!(u8, U8);
impl_from_ref!(u16, U16);
impl_from_ref!(u32, U32);
impl_from_ref!(u64, U64);
impl_from_ref!(u128, U128);
impl_from_ref!(usize, USize);
impl_from_ref!(String, Str);
impl_from_ref!(char, Char);

impl From<&str> for Solution {
    fn from(sol: &str) -> Self {
        Self::Str(sol.to_owned())
    }
}
