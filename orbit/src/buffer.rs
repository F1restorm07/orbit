use starlink::types::Size;
use core::ops::Range;
use core::ops::Mul;
use typenum::{ Prod, U, Const, ToUInt };
use generic_array::{ ArrayLength, GenericArray };

// type alias for the size of a terminal buffer
type BufSize<const WIDTH: usize, const HEIGHT: usize>
= Prod<U<WIDTH>, U<HEIGHT>>;

/// a single character slot on a terminal screen
#[derive(Default)]
pub struct TermCell(char);

/// a collection of terminal character slots
pub struct Buffer<const WIDTH: usize, const HEIGHT: usize>
where
    Const<WIDTH>: ToUInt,
    Const<HEIGHT>: ToUInt,
    U<WIDTH>: Mul<U<HEIGHT>>, // enable multiplication with HEIGHT
    BufSize<WIDTH, HEIGHT>: ArrayLength
{
    // area: Size, // ??: is this needed here (may use for binding to a widget space)
    chars: GenericArray<TermCell, BufSize<WIDTH, HEIGHT>>
}

impl<const WIDTH: usize, const HEIGHT: usize> Buffer<WIDTH, HEIGHT>
where
    Const<WIDTH>: ToUInt,
    Const<HEIGHT>: ToUInt,
    U<WIDTH>: Mul<U<HEIGHT>>,
    Prod<U<WIDTH>, U<HEIGHT>>: ArrayLength
{
    pub fn new() -> Self {
        Self {
            // area: Size::new(width as u32, height as u32),
            chars: GenericArray::default()
        }
    }
    /// get a character slot within the buffer, supports exclusive ranges
    pub fn get_char(&self, offset: Range<usize>) -> &[TermCell] {
        &self.chars[offset]
    }
    /// set a character slot in the buffer, supports exclusive ranges
    pub fn set_char(&mut self, c: char, offset: Range<usize>) {
        // needed because offset is not const
        let _ = offset.map(|d| self.chars[d] = TermCell(c));
    }
}

pub fn get_char_offset(x: u16, y: u16) -> usize {
        ((y * x) + x) as usize
    }
