#![warn(unreachable_pub)]

//! # Material Design Colors for Rust
//!
//! Easily get color variants based on the Material Design Pallete. All colors
//! are represented as 0xRRGGBB with the first two bytes not used.
//!
//! To access a color you may use the `const fn` under [`MatColor`] or access
//! the `const` arrays that are publicly available.

/// Main access point to all colors.
pub struct MatColor;

/// All red color accents
pub const MAT_COLORS_RED: [u32; 14] = [
    0xFFEBEE, 0xFFCDD2, 0xEF9A9A, 0xE57373, 0xEF5350, 0xF44336, 0xE53935, 0xD32F2F, 0xC62828,
    0xB71C1C, 0xFF8A80, 0xFF5252, 0xFF1744, 0xD50000,
];

/// All the main color variants by name in the 2014 Material Design Pallete.
pub enum MatColorVariant {
    Red,
    Pink,
    Purple,
    DeepPurple,
    Indigo,
    Blue,
    LightBlue,
    Cyan,
    Teal,
    Green,
    LightGreen,
    Lime,
    Yellow,
    Amber,
    Orange,
    DeepOrange,
    Brown,
    Gray,
    BlueGray,
    Black,
    White,
}

/// All the color accents in the 2014 Material Design Pallete.
pub enum MatColorAccent {
    C50,
    C100,
    C200,
    C300,
    C400,
    C500,
    C600,
    C700,
    C800,
    C900,
    A100,
    A200,
    A400,
    A700,
}

/// Red color and its accents
pub struct MatColorRed;

#[rustfmt::skip]
impl MatColorRed {
    /// Red\[50\] -> 0xFFEBEE
    /// <div style="padding: 10px; background-color: #FFEBEE"></div>
    pub const fn c50() -> u32 { 0xFFEBEE }
    /// Red\[100\] -> 0xFFCDD2
    /// <div style="padding: 10px; background-color: #FFCDD2"></div>
    pub const fn c100() -> u32 { 0xFFCDD2 }
    /// Red\[200\] -> 0xEF9A9A
    /// <div style="padding: 10px; background-color: #EF9A9A"></div>
    pub const fn c200() -> u32 { 0xEF9A9A }
    /// Red\[300\] -> 0xE57373
    /// <div style="padding: 10px; background-color: #E57373"></div>
    pub const fn c300() -> u32 { 0xE57373 }
    /// Red\[400\] -> 0xEF5350
    /// <div style="padding: 10px; background-color: #EF5350"></div>
    pub const fn c400() -> u32 { 0xEF5350 }
    /// Red\[500\] -> 0xF44336
    /// <div style="padding: 10px; background-color: #F44336"></div>
    pub const fn c500() -> u32 { 0xF44336 }
    /// Red\[600\] -> 0xE53935
    /// <div style="padding: 10px; background-color: #E53935"></div>
    pub const fn c600() -> u32 { 0xE53935 }
    /// Red\[700\] -> 0xD32F2F
    /// <div style="padding: 10px; background-color: #D32F2F"></div>
    pub const fn c700() -> u32 { 0xD32F2F }
    /// Red\[800\] -> 0xC62828
    /// <div style="padding: 10px; background-color: #C62828"></div>
    pub const fn c800() -> u32 { 0xC62828 }
    /// Red\[900\] -> 0xB71C1C
    /// <div style="padding: 10px; background-color: #B71C1C"></div>
    pub const fn c900() -> u32 { 0xB71C1C }
    /// Red\[100\] -> 0xFF8A80
    /// <div style="padding: 10px; background-color: #FF8A80"></div>
    pub const fn a100() -> u32 { 0xFF8A80 }
    /// Red\[200\] -> 0xFF5252
    /// <div style="padding: 10px; background-color: #FF5252"></div>
    pub const fn a200() -> u32 { 0xFF5252 }
    /// Red\[400\] -> 0xFF1744
    /// <div style="padding: 10px; background-color: #FF1744"></div>
    pub const fn a400() -> u32 { 0xFF1744 }
    /// Red\[700\] -> 0xD50000
    /// <div style="padding: 10px; background-color: #D50000"></div>
    pub const fn a700() -> u32 { 0xD50000 }
}

/// Pink color and its accents
pub struct MatColorPink;

#[rustfmt::skip]
impl MatColorPink {
    /// Pink\[50\] -> 0xFCE4EC
    /// <div style="padding: 10px; background-color: #FCE4EC"></div>
    pub const fn c50() -> u32 { 0xFCE4EC }
    /// Pink\[100\] -> 0xF8BBD0
    /// <div style="padding: 10px; background-color: #F8BBD0"></div>
    pub const fn c100() -> u32 { 0xF8BBD0 }
    /// Pink\[200\] -> 0xF48FB1
    /// <div style="padding: 10px; background-color: #F48FB1"></div>
    pub const fn c200() -> u32 { 0xF48FB1 }
    /// Pink\[300\] -> 0xF06292
    /// <div style="padding: 10px; background-color: #F06292"></div>
    pub const fn c300() -> u32 { 0xF06292 }
    /// Pink\[400\] -> 0xEC407A
    /// <div style="padding: 10px; background-color: #EC407A"></div>
    pub const fn c400() -> u32 { 0xEC407A }
    /// Pink\[500\] -> 0xE91E63
    /// <div style="padding: 10px; background-color: #E91E63"></div>
    pub const fn c500() -> u32 { 0xE91E63 }
    /// Pink\[600\] -> 0xD81B60
    /// <div style="padding: 10px; background-color: #D81B60"></div>
    pub const fn c600() -> u32 { 0xD81B60 }
    /// Pink\[700\] -> 0xC2185B
    /// <div style="padding: 10px; background-color: #C2185B"></div>
    pub const fn c700() -> u32 { 0xC2185B }
    /// Pink\[800\] -> 0xAD1457
    /// <div style="padding: 10px; background-color: #AD1457"></div>
    pub const fn c800() -> u32 { 0xAD1457 }
    /// Pink\[900\] -> 0x880E4F
    /// <div style="padding: 10px; background-color: #880E4F"></div>
    pub const fn c900() -> u32 { 0x880E4F }
    /// Pink\[A100\] -> 0xFF80AB
    /// <div style="padding: 10px; background-color: #FF80AB"></div>
    pub const fn a100() -> u32 { 0xFF80AB }
    /// Pink\[A200\] -> 0xFF4081
    /// <div style="padding: 10px; background-color: #FF4081"></div>
    pub const fn a200() -> u32 { 0xFF4081 }
    /// Pink\[A400\] -> 0xF50057
    /// <div style="padding: 10px; background-color: #F50057"></div>
    pub const fn a400() -> u32 { 0xF50057 }
    /// Pink\[A700\] -> 0xC51162
    /// <div style="padding: 10px; background-color: #C51162"></div>
    pub const fn a700() -> u32 { 0xC51162 }
}

/// Purple color and its accents
pub struct MatColorPurple;

#[rustfmt::skip]
impl MatColorPurple {
    /// Purple\[50\] -> 0xF3E5F5
    /// <div style="padding: 10px; background-color: #F3E5F5"></div>
    pub const fn c50() -> u32 { 0xF3E5F5 }
    /// Purple\[100\] -> 0xE1BEE7
    /// <div style="padding: 10px; background-color: #E1BEE7"></div>
    pub const fn c100() -> u32 { 0xE1BEE7 }
    /// Purple\[200\] -> 0xCE93D8
    /// <div style="padding: 10px; background-color: #CE93D8"></div>
    pub const fn c200() -> u32 { 0xCE93D8 }
    /// Purple\[300\] -> 0xBA68C8
    /// <div style="padding: 10px; background-color: #BA68C8"></div>
    pub const fn c300() -> u32 { 0xBA68C8 }
    /// Purple\[400\] -> 0xAB47BC
    /// <div style="padding: 10px; background-color: #AB47BC"></div>
    pub const fn c400() -> u32 { 0xAB47BC }
    /// Purple\[500\] -> 0x9C27B0
    /// <div style="padding: 10px; background-color: #9C27B0"></div>
    pub const fn c500() -> u32 { 0x9C27B0 }
    /// Purple\[600\] -> 0x8E24AA
    /// <div style="padding: 10px; background-color: #8E24AA"></div>
    pub const fn c600() -> u32 { 0x8E24AA }
    /// Purple\[700\] -> 0x7B1FA2
    /// <div style="padding: 10px; background-color: #7B1FA2"></div>
    pub const fn c700() -> u32 { 0x7B1FA2 }
    /// Purple\[800\] -> 0x6A1B9A
    /// <div style="padding: 10px; background-color: #6A1B9A"></div>
    pub const fn c800() -> u32 { 0x6A1B9A }
    /// Purple\[900\] -> 0x4A148C
    /// <div style="padding: 10px; background-color: #4A148C"></div>
    pub const fn c900() -> u32 { 0x4A148C }
    /// Purple\[A100\] -> 0xEA80FC
    /// <div style="padding: 10px; background-color: #EA80FC"></div>
    pub const fn a100() -> u32 { 0xEA80FC }
    /// Purple\[A200\] -> 0xE040FB
    /// <div style="padding: 10px; background-color: #E040FB"></div>
    pub const fn a200() -> u32 { 0xE040FB }
    /// Purple\[A400\] -> 0xD500F9
    /// <div style="padding: 10px; background-color: #D500F9"></div>
    pub const fn a400() -> u32 { 0xD500F9 }
    /// Purple\[A700\] -> 0xAA00FF
    /// <div style="padding: 10px; background-color: #AA00FF"></div>
    pub const fn a700() -> u32 { 0xAA00FF }
}

/// Deep Purple color and its accents
pub struct MatColorDeepPurple;

#[rustfmt::skip]
impl MatColorDeepPurple {
    /// DeepPurple\[50\] -> 0xEDE7F6
    /// <div style="padding: 10px; background-color: #EDE7F6"></div>
    pub const fn c50() -> u32 { 0xEDE7F6 }
    /// DeepPurple\[100\] -> 0xD1C4E9
    /// <div style="padding: 10px; background-color: #D1C4E9"></div>
    pub const fn c100() -> u32 { 0xD1C4E9 }
    /// DeepPurple\[200\] -> 0xB39DDB
    /// <div style="padding: 10px; background-color: #B39DDB"></div>
    pub const fn c200() -> u32 { 0xB39DDB }
    /// DeepPurple\[300\] -> 0x9575CD
    /// <div style="padding: 10px; background-color: #9575CD"></div>
    pub const fn c300() -> u32 { 0x9575CD }
    /// DeepPurple\[400\] -> 0x7E57C2
    /// <div style="padding: 10px; background-color: #7E57C2"></div>
    pub const fn c400() -> u32 { 0x7E57C2 }
    /// DeepPurple\[500\] -> 0x673AB7
    /// <div style="padding: 10px; background-color: #673AB7"></div>
    pub const fn c500() -> u32 { 0x673AB7 }
    /// DeepPurple\[600\] -> 0x5E35B1
    /// <div style="padding: 10px; background-color: #5E35B1"></div>
    pub const fn c600() -> u32 { 0x5E35B1 }
    /// DeepPurple\[700\] -> 0x512DA8
    /// <div style="padding: 10px; background-color: #512DA8"></div>
    pub const fn c700() -> u32 { 0x512DA8 }
    /// DeepPurple\[800\] -> 0x4527A0
    /// <div style="padding: 10px; background-color: #4527A0"></div>
    pub const fn c800() -> u32 { 0x4527A0 }
    /// DeepPurple\[900\] -> 0x311B92
    /// <div style="padding: 10px; background-color: #311B92"></div>
    pub const fn c900() -> u32 { 0x311B92 }
    /// DeepPurple\[A100\] -> 0xB388FF
    /// <div style="padding: 10px; background-color: #B388FF"></div>
    pub const fn a100() -> u32 { 0xB388FF }
    /// DeepPurple\[A200\] -> 0x7C4DFF
    /// <div style="padding: 10px; background-color: #7C4DFF"></div>
    pub const fn a200() -> u32 { 0x7C4DFF }
    /// DeepPurple\[A400\] -> 0x651FFF
    /// <div style="padding: 10px; background-color: #651FFF"></div>
    pub const fn a400() -> u32 { 0x651FFF }
    /// DeepPurple\[A700\] -> 0x6200EA
    /// <div style="padding: 10px; background-color: #6200EA"></div>
    pub const fn a700() -> u32 { 0x6200EA }
}

/// Indigo color and its accents
pub struct MatColorIndigo;

#[rustfmt::skip]
impl MatColorIndigo {
    /// Indigo\[50\] -> 0xE8EAF6
    /// <div style="padding: 10px; background-color: #E8EAF6"></div>
    pub const fn c50() -> u32 { 0xE8EAF6 }
    /// Indigo\[100\] -> 0xC5CAE9
    /// <div style="padding: 10px; background-color: #C5CAE9"></div>
    pub const fn c100() -> u32 { 0xC5CAE9 }
    /// Indigo\[200\] -> 0x9FA8DA
    /// <div style="padding: 10px; background-color: #9FA8DA"></div>
    pub const fn c200() -> u32 { 0x9FA8DA }
    /// Indigo\[300\] -> 0x7986CB
    /// <div style="padding: 10px; background-color: #7986CB"></div>
    pub const fn c300() -> u32 { 0x7986CB }
    /// Indigo\[400\] -> 0x5C6BC0
    /// <div style="padding: 10px; background-color: #5C6BC0"></div>
    pub const fn c400() -> u32 { 0x5C6BC0 }
    /// Indigo\[500\] -> 0x3F51B5
    /// <div style="padding: 10px; background-color: #3F51B5"></div>
    pub const fn c500() -> u32 { 0x3F51B5 }
    /// Indigo\[600\] -> 0x3949AB
    /// <div style="padding: 10px; background-color: #3949AB"></div>
    pub const fn c600() -> u32 { 0x3949AB }
    /// Indigo\[700\] -> 0x303F9F
    /// <div style="padding: 10px; background-color: #303F9F"></div>
    pub const fn c700() -> u32 { 0x303F9F }
    /// Indigo\[800\] -> 0x283593
    /// <div style="padding: 10px; background-color: #283593"></div>
    pub const fn c800() -> u32 { 0x283593 }
    /// Indigo\[900\] -> 0x1A237E
    /// <div style="padding: 10px; background-color: #1A237E"></div>
    pub const fn c900() -> u32 { 0x1A237E }
    /// Indigo\[A100\] -> 0x8C9EFF
    /// <div style="padding: 10px; background-color: #8C9EFF"></div>
    pub const fn a100() -> u32 { 0x8C9EFF }
    /// Indigo\[A200\] -> 0x536DFE
    /// <div style="padding: 10px; background-color: #536DFE"></div>
    pub const fn a200() -> u32 { 0x536DFE }
    /// Indigo\[A400\] -> 0x3D5AFE
    /// <div style="padding: 10px; background-color: #3D5AFE"></div>
    pub const fn a400() -> u32 { 0x3D5AFE }
    /// Indigo\[A700\] -> 0x304FFE
    /// <div style="padding: 10px; background-color: #304FFE"></div>
    pub const fn a700() -> u32 { 0x304FFE }
}

/// Blue color and its accents
pub struct MatColorBlue;

#[rustfmt::skip]
impl MatColorBlue {
    /// Blue\[50\] -> 0xE3F2FD
    /// <div style="padding: 10px; background-color: #E3F2FD"></div>
    pub const fn c50() -> u32 { 0xE3F2FD }
    /// Blue\[100\] -> 0xBBDEFB
    /// <div style="padding: 10px; background-color: #BBDEFB"></div>
    pub const fn c100() -> u32 { 0xBBDEFB }
    /// Blue\[200\] -> 0x90CAF9
    /// <div style="padding: 10px; background-color: #90CAF9"></div>
    pub const fn c200() -> u32 { 0x90CAF9 }
    /// Blue\[300\] -> 0x64B5F6
    /// <div style="padding: 10px; background-color: #64B5F6"></div>
    pub const fn c300() -> u32 { 0x64B5F6 }
    /// Blue\[400\] -> 0x42A5F5
    /// <div style="padding: 10px; background-color: #42A5F5"></div>
    pub const fn c400() -> u32 { 0x42A5F5 }
    /// Blue\[500\] -> 0x2196F3
    /// <div style="padding: 10px; background-color: #2196F3"></div>
    pub const fn c500() -> u32 { 0x2196F3 }
    /// Blue\[600\] -> 0x1E88E5
    /// <div style="padding: 10px; background-color: #1E88E5"></div>
    pub const fn c600() -> u32 { 0x1E88E5 }
    /// Blue\[700\] -> 0x1976D2
    /// <div style="padding: 10px; background-color: #1976D2"></div>
    pub const fn c700() -> u32 { 0x1976D2 }
    /// Blue\[800\] -> 0x1565C0
    /// <div style="padding: 10px; background-color: #1565C0"></div>
    pub const fn c800() -> u32 { 0x1565C0 }
    /// Blue\[900\] -> 0x0D47A1
    /// <div style="padding: 10px; background-color: #0D47A1"></div>
    pub const fn c900() -> u32 { 0x0D47A1 }
    /// Blue\[A100\] -> 0x82B1FF
    /// <div style="padding: 10px; background-color: #82B1FF"></div>
    pub const fn a100() -> u32 { 0x82B1FF }
    /// Blue\[A200\] -> 0x448AFF
    /// <div style="padding: 10px; background-color: #448AFF"></div>
    pub const fn a200() -> u32 { 0x448AFF }
    /// Blue\[A400\] -> 0x2979FF
    /// <div style="padding: 10px; background-color: #2979FF"></div>
    pub const fn a400() -> u32 { 0x2979FF }
    /// Blue\[A700\] -> 0x2962FF
    /// <div style="padding: 10px; background-color: #2962FF"></div>
    pub const fn a700() -> u32 { 0x2962FF }
}

/// Light Blue color and its accents
pub struct MatColorLightBlue;

#[rustfmt::skip]
impl MatColorLightBlue {
    /// LightBlue\[50\] -> 0xE1F5FE
    /// <div style="padding: 10px; background-color: #E1F5FE"></div>
    pub const fn c50() -> u32 { 0xE1F5FE }
    /// LightBlue\[100\] -> 0xB3E5FC
    /// <div style="padding: 10px; background-color: #B3E5FC"></div>
    pub const fn c100() -> u32 { 0xB3E5FC }
    /// LightBlue\[200\] -> 0x81D4FA
    /// <div style="padding: 10px; background-color: #81D4FA"></div>
    pub const fn c200() -> u32 { 0x81D4FA }
    /// LightBlue\[300\] -> 0x4FC3F7
    /// <div style="padding: 10px; background-color: #4FC3F7"></div>
    pub const fn c300() -> u32 { 0x4FC3F7 }
    /// LightBlue\[400\] -> 0x29B6F6
    /// <div style="padding: 10px; background-color: #29B6F6"></div>
    pub const fn c400() -> u32 { 0x29B6F6 }
    /// LightBlue\[500\] -> 0x03A9F4
    /// <div style="padding: 10px; background-color: #03A9F4"></div>
    pub const fn c500() -> u32 { 0x03A9F4 }
    /// LightBlue\[600\] -> 0x039BE5
    /// <div style="padding: 10px; background-color: #039BE5"></div>
    pub const fn c600() -> u32 { 0x039BE5 }
    /// LightBlue\[700\] -> 0x0288D1
    /// <div style="padding: 10px; background-color: #0288D1"></div>
    pub const fn c700() -> u32 { 0x0288D1 }
    /// LightBlue\[800\] -> 0x0277BD
    /// <div style="padding: 10px; background-color: #0277BD"></div>
    pub const fn c800() -> u32 { 0x0277BD }
    /// LightBlue\[900\] -> 0x01579B
    /// <div style="padding: 10px; background-color: #01579B"></div>
    pub const fn c900() -> u32 { 0x01579B }
    /// LightBlue\[A100\] -> 0x80D8FF
    /// <div style="padding: 10px; background-color: #80D8FF"></div>
    pub const fn a100() -> u32 { 0x80D8FF }
    /// LightBlue\[A200\] -> 0x40C4FF
    /// <div style="padding: 10px; background-color: #40C4FF"></div>
    pub const fn a200() -> u32 { 0x40C4FF }
    /// LightBlue\[A400\] -> 0x00B0FF
    /// <div style="padding: 10px; background-color: #00B0FF"></div>
    pub const fn a400() -> u32 { 0x00B0FF }
    /// LightBlue\[A700\] -> 0x0091EA
    /// <div style="padding: 10px; background-color: #0091EA"></div>
    pub const fn a700() -> u32 { 0x0091EA }
}

/// Cyan color and its accents
pub struct MatColorCyan;

#[rustfmt::skip]
impl MatColorCyan {
    /// Cyan\[50\] -> 0xE0F7FA
    /// <div style="padding: 10px; background-color: #E0F7FA"></div>
    pub const fn c50() -> u32 { 0xE0F7FA }
    /// Cyan\[100\] -> 0xB2EBF2
    /// <div style="padding: 10px; background-color: #B2EBF2"></div>
    pub const fn c100() -> u32 { 0xB2EBF2 }
    /// Cyan\[200\] -> 0x80DEEA
    /// <div style="padding: 10px; background-color: #80DEEA"></div>
    pub const fn c200() -> u32 { 0x80DEEA }
    /// Cyan\[300\] -> 0x4DD0E1
    /// <div style="padding: 10px; background-color: #4DD0E1"></div>
    pub const fn c300() -> u32 { 0x4DD0E1 }
    /// Cyan\[400\] -> 0x26C6DA
    /// <div style="padding: 10px; background-color: #26C6DA"></div>
    pub const fn c400() -> u32 { 0x26C6DA }
    /// Cyan\[500\] -> 0x00BCD4
    /// <div style="padding: 10px; background-color: #00BCD4"></div>
    pub const fn c500() -> u32 { 0x00BCD4 }
    /// Cyan\[600\] -> 0x00ACC1
    /// <div style="padding: 10px; background-color: #00ACC1"></div>
    pub const fn c600() -> u32 { 0x00ACC1 }
    /// Cyan\[700\] -> 0x0097A7
    /// <div style="padding: 10px; background-color: #0097A7"></div>
    pub const fn c700() -> u32 { 0x0097A7 }
    /// Cyan\[800\] -> 0x00838F
    /// <div style="padding: 10px; background-color: #00838F"></div>
    pub const fn c800() -> u32 { 0x00838F }
    /// Cyan\[900\] -> 0x006064
    /// <div style="padding: 10px; background-color: #006064"></div>
    pub const fn c900() -> u32 { 0x006064 }
    /// Cyan\[A100\] -> 0x84FFFF
    /// <div style="padding: 10px; background-color: #84FFFF"></div>
    pub const fn a100() -> u32 { 0x84FFFF }
    /// Cyan\[A200\] -> 0x18FFFF
    /// <div style="padding: 10px; background-color: #18FFFF"></div>
    pub const fn a200() -> u32 { 0x18FFFF }
    /// Cyan\[A400\] -> 0x00E5FF
    /// <div style="padding: 10px; background-color: #00E5FF"></div>
    pub const fn a400() -> u32 { 0x00E5FF }
    /// Cyan\[A700\] -> 0x00B8D4
    /// <div style="padding: 10px; background-color: #00B8D4"></div>
    pub const fn a700() -> u32 { 0x00B8D4 }
}

/// Teal color and its accents
pub struct MatColorTeal;

#[rustfmt::skip]
impl MatColorTeal {
    /// Teal\[50\] -> 0xE0F2F1
    /// <div style="padding: 10px; background-color: #E0F2F1"></div>
    pub const fn c50() -> u32 { 0xE0F2F1 }
    /// Teal\[100\] -> 0xB2DFDB
    /// <div style="padding: 10px; background-color: #B2DFDB"></div>
    pub const fn c100() -> u32 { 0xB2DFDB }
    /// Teal\[200\] -> 0x80CBC4
    /// <div style="padding: 10px; background-color: #80CBC4"></div>
    pub const fn c200() -> u32 { 0x80CBC4 }
    /// Teal\[300\] -> 0x4DB6AC
    /// <div style="padding: 10px; background-color: #4DB6AC"></div>
    pub const fn c300() -> u32 { 0x4DB6AC }
    /// Teal\[400\] -> 0x26A69A
    /// <div style="padding: 10px; background-color: #26A69A"></div>
    pub const fn c400() -> u32 { 0x26A69A }
    /// Teal\[500\] -> 0x009688
    /// <div style="padding: 10px; background-color: #009688"></div>
    pub const fn c500() -> u32 { 0x009688 }
    /// Teal\[600\] -> 0x00897B
    /// <div style="padding: 10px; background-color: #00897B"></div>
    pub const fn c600() -> u32 { 0x00897B }
    /// Teal\[700\] -> 0x00796B
    /// <div style="padding: 10px; background-color: #00796B"></div>
    pub const fn c700() -> u32 { 0x00796B }
    /// Teal\[800\] -> 0x00695C
    /// <div style="padding: 10px; background-color: #00695C"></div>
    pub const fn c800() -> u32 { 0x00695C }
    /// Teal\[900\] -> 0x004D40
    /// <div style="padding: 10px; background-color: #004D40"></div>
    pub const fn c900() -> u32 { 0x004D40 }
    /// Teal\[A100\] -> 0xA7FFEB
    /// <div style="padding: 10px; background-color: #A7FFEB"></div>
    pub const fn a100() -> u32 { 0xA7FFEB }
    /// Teal\[A200\] -> 0x64FFDA
    /// <div style="padding: 10px; background-color: #64FFDA"></div>
    pub const fn a200() -> u32 { 0x64FFDA }
    /// Teal\[A400\] -> 0x1DE9B6
    /// <div style="padding: 10px; background-color: #1DE9B6"></div>
    pub const fn a400() -> u32 { 0x1DE9B6 }
    /// Teal\[A700\] -> 0x00BFA5
    /// <div style="padding: 10px; background-color: #00BFA5"></div>
    pub const fn a700() -> u32 { 0x00BFA5 }
}

/// Green color and its accents
pub struct MatColorGreen;

#[rustfmt::skip]
impl MatColorGreen {
    /// Green\[50\] -> 0xE8F5E9
    /// <div style="padding: 10px; background-color: #E8F5E9"></div>
    pub const fn c50() -> u32 { 0xE8F5E9 }
    /// Green\[100\] -> 0xC8E6C9
    /// <div style="padding: 10px; background-color: #C8E6C9"></div>
    pub const fn c100() -> u32 { 0xC8E6C9 }
    /// Green\[200\] -> 0xA5D6A7
    /// <div style="padding: 10px; background-color: #A5D6A7"></div>
    pub const fn c200() -> u32 { 0xA5D6A7 }
    /// Green\[300\] -> 0x81C784
    /// <div style="padding: 10px; background-color: #81C784"></div>
    pub const fn c300() -> u32 { 0x81C784 }
    /// Green\[400\] -> 0x66BB6A
    /// <div style="padding: 10px; background-color: #66BB6A"></div>
    pub const fn c400() -> u32 { 0x66BB6A }
    /// Green\[500\] -> 0x4CAF50
    /// <div style="padding: 10px; background-color: #4CAF50"></div>
    pub const fn c500() -> u32 { 0x4CAF50 }
    /// Green\[600\] -> 0x43A047
    /// <div style="padding: 10px; background-color: #43A047"></div>
    pub const fn c600() -> u32 { 0x43A047 }
    /// Green\[700\] -> 0x388E3C
    /// <div style="padding: 10px; background-color: #388E3C"></div>
    pub const fn c700() -> u32 { 0x388E3C }
    /// Green\[800\] -> 0x2E7D32
    /// <div style="padding: 10px; background-color: #2E7D32"></div>
    pub const fn c800() -> u32 { 0x2E7D32 }
    /// Green\[900\] -> 0x1B5E20
    /// <div style="padding: 10px; background-color: #1B5E20"></div>
    pub const fn c900() -> u32 { 0x1B5E20 }
    /// Green\[A100\] -> 0xB9F6CA
    /// <div style="padding: 10px; background-color: #B9F6CA"></div>
    pub const fn a100() -> u32 { 0xB9F6CA }
    /// Green\[A200\] -> 0x69F0AE
    /// <div style="padding: 10px; background-color: #69F0AE"></div>
    pub const fn a200() -> u32 { 0x69F0AE }
    /// Green\[A400\] -> 0x00E676
    /// <div style="padding: 10px; background-color: #00E676"></div>
    pub const fn a400() -> u32 { 0x00E676 }
    /// Green\[A700\] -> 0x00C853
    /// <div style="padding: 10px; background-color: #00C853"></div>
    pub const fn a700() -> u32 { 0x00C853 }
}

/// Light Green color and its accents
pub struct MatColorLightGreen;

#[rustfmt::skip]
impl MatColorLightGreen {
    /// LightGreen\[50\] -> 0xF1F8E9
    /// <div style="padding: 10px; background-color: #F1F8E9"></div>
    pub const fn c50() -> u32 { 0xF1F8E9 }
    /// LightGreen\[100\] -> 0xDCEDC8
    /// <div style="padding: 10px; background-color: #DCEDC8"></div>
    pub const fn c100() -> u32 { 0xDCEDC8 }
    /// LightGreen\[200\] -> 0xC5E1A5
    /// <div style="padding: 10px; background-color: #C5E1A5"></div>
    pub const fn c200() -> u32 { 0xC5E1A5 }
    /// LightGreen\[300\] -> 0xAED581
    /// <div style="padding: 10px; background-color: #AED581"></div>
    pub const fn c300() -> u32 { 0xAED581 }
    /// LightGreen\[400\] -> 0x9CCC65
    /// <div style="padding: 10px; background-color: #9CCC65"></div>
    pub const fn c400() -> u32 { 0x9CCC65 }
    /// LightGreen\[500\] -> 0x8BC34A
    /// <div style="padding: 10px; background-color: #8BC34A"></div>
    pub const fn c500() -> u32 { 0x8BC34A }
    /// LightGreen\[600\] -> 0x7CB342
    /// <div style="padding: 10px; background-color: #7CB342"></div>
    pub const fn c600() -> u32 { 0x7CB342 }
    /// LightGreen\[700\] -> 0x689F38
    /// <div style="padding: 10px; background-color: #689F38"></div>
    pub const fn c700() -> u32 { 0x689F38 }
    /// LightGreen\[800\] -> 0x558B2F
    /// <div style="padding: 10px; background-color: #558B2F"></div>
    pub const fn c800() -> u32 { 0x558B2F }
    /// LightGreen\[900\] -> 0x33691E
    /// <div style="padding: 10px; background-color: #33691E"></div>
    pub const fn c900() -> u32 { 0x33691E }
    /// LightGreen\[A100\] -> 0xCCFF90
    /// <div style="padding: 10px; background-color: #CCFF90"></div>
    pub const fn a100() -> u32 { 0xCCFF90 }
    /// LightGreen\[A200\] -> 0xB2FF59
    /// <div style="padding: 10px; background-color: #B2FF59"></div>
    pub const fn a200() -> u32 { 0xB2FF59 }
    /// LightGreen\[A400\] -> 0x76FF03
    /// <div style="padding: 10px; background-color: #76FF03"></div>
    pub const fn a400() -> u32 { 0x76FF03 }
    /// LightGreen\[A700\] -> 0x64DD17
    /// <div style="padding: 10px; background-color: #64DD17"></div>
    pub const fn a700() -> u32 { 0x64DD17 }
}

/// Lime color and its accents
pub struct MatColorLime;

#[rustfmt::skip]
impl MatColorLime {
    /// Lime\[50\] -> 0xF9FBE7
    /// <div style="padding: 10px; background-color: #F9FBE7"></div>
    pub const fn c50() -> u32 { 0xF9FBE7 }
    /// Lime\[100\] -> 0xF0F4C3
    /// <div style="padding: 10px; background-color: #F0F4C3"></div>
    pub const fn c100() -> u32 { 0xF0F4C3 }
    /// Lime\[200\] -> 0xE6EE9C
    /// <div style="padding: 10px; background-color: #E6EE9C"></div>
    pub const fn c200() -> u32 { 0xE6EE9C }
    /// Lime\[300\] -> 0xDCE775
    /// <div style="padding: 10px; background-color: #DCE775"></div>
    pub const fn c300() -> u32 { 0xDCE775 }
    /// Lime\[400\] -> 0xD4E157
    /// <div style="padding: 10px; background-color: #D4E157"></div>
    pub const fn c400() -> u32 { 0xD4E157 }
    /// Lime\[500\] -> 0xCDDC39
    /// <div style="padding: 10px; background-color: #CDDC39"></div>
    pub const fn c500() -> u32 { 0xCDDC39 }
    /// Lime\[600\] -> 0xC0CA33
    /// <div style="padding: 10px; background-color: #C0CA33"></div>
    pub const fn c600() -> u32 { 0xC0CA33 }
    /// Lime\[700\] -> 0xAFB42B
    /// <div style="padding: 10px; background-color: #AFB42B"></div>
    pub const fn c700() -> u32 { 0xAFB42B }
    /// Lime\[800\] -> 0x9E9D24
    /// <div style="padding: 10px; background-color: #9E9D24"></div>
    pub const fn c800() -> u32 { 0x9E9D24 }
    /// Lime\[900\] -> 0x827717
    /// <div style="padding: 10px; background-color: #827717"></div>
    pub const fn c900() -> u32 { 0x827717 }
    /// Lime\[A100\] -> 0xF4FF81
    /// <div style="padding: 10px; background-color: #F4FF81"></div>
    pub const fn a100() -> u32 { 0xF4FF81 }
    /// Lime\[A200\] -> 0xEEFF41
    /// <div style="padding: 10px; background-color: #EEFF41"></div>
    pub const fn a200() -> u32 { 0xEEFF41 }
    /// Lime\[A400\] -> 0xC6FF00
    /// <div style="padding: 10px; background-color: #C6FF00"></div>
    pub const fn a400() -> u32 { 0xC6FF00 }
    /// Lime\[A700\] -> 0xAEEA00
    /// <div style="padding: 10px; background-color: #AEEA00"></div>
    pub const fn a700() -> u32 { 0xAEEA00 }
}

/// Yellow color and its accents
pub struct MatColorYellow;

#[rustfmt::skip]
impl MatColorYellow {
    /// Yellow\[50\] -> 0xFFFDE7
    /// <div style="padding: 10px; background-color: #FFFDE7"></div>
    pub const fn c50() -> u32 { 0xFFFDE7 }
    /// Yellow\[100\] -> 0xFFF9C4
    /// <div style="padding: 10px; background-color: #FFF9C4"></div>
    pub const fn c100() -> u32 { 0xFFF9C4 }
    /// Yellow\[200\] -> 0xFFF59D
    /// <div style="padding: 10px; background-color: #FFF59D"></div>
    pub const fn c200() -> u32 { 0xFFF59D }
    /// Yellow\[300\] -> 0xFFF176
    /// <div style="padding: 10px; background-color: #FFF176"></div>
    pub const fn c300() -> u32 { 0xFFF176 }
    /// Yellow\[400\] -> 0xFFEE58
    /// <div style="padding: 10px; background-color: #FFEE58"></div>
    pub const fn c400() -> u32 { 0xFFEE58 }
    /// Yellow\[500\] -> 0xFFEB3B
    /// <div style="padding: 10px; background-color: #FFEB3B"></div>
    pub const fn c500() -> u32 { 0xFFEB3B }
    /// Yellow\[600\] -> 0xFDD835
    /// <div style="padding: 10px; background-color: #FDD835"></div>
    pub const fn c600() -> u32 { 0xFDD835 }
    /// Yellow\[700\] -> 0xFBC02D
    /// <div style="padding: 10px; background-color: #FBC02D"></div>
    pub const fn c700() -> u32 { 0xFBC02D }
    /// Yellow\[800\] -> 0xF9A825
    /// <div style="padding: 10px; background-color: #F9A825"></div>
    pub const fn c800() -> u32 { 0xF9A825 }
    /// Yellow\[900\] -> 0xF57F17
    /// <div style="padding: 10px; background-color: #F57F17"></div>
    pub const fn c900() -> u32 { 0xF57F17 }
    /// Yellow\[A100\] -> 0xFFFF8D
    /// <div style="padding: 10px; background-color: #FFFF8D"></div>
    pub const fn a100() -> u32 { 0xFFFF8D }
    /// Yellow\[A200\] -> 0xFFFF00
    /// <div style="padding: 10px; background-color: #FFFF00"></div>
    pub const fn a200() -> u32 { 0xFFFF00 }
    /// Yellow\[A400\] -> 0xFFEA00
    /// <div style="padding: 10px; background-color: #FFEA00"></div>
    pub const fn a400() -> u32 { 0xFFEA00 }
    /// Yellow\[A700\] -> 0xFFD600
    /// <div style="padding: 10px; background-color: #FFD600"></div>
    pub const fn a700() -> u32 { 0xFFD600 }
}

/// Amber color and its accents
pub struct MatColorAmber;

#[rustfmt::skip]
impl MatColorAmber {
    /// Amber\[50\] -> 0xFFF8E1
    /// <div style="padding: 10px; background-color: #FFF8E1"></div>
    pub const fn c50() -> u32 { 0xFFF8E1 }
    /// Amber\[100\] -> 0xFFECB3
    /// <div style="padding: 10px; background-color: #FFECB3"></div>
    pub const fn c100() -> u32 { 0xFFECB3 }
    /// Amber\[200\] -> 0xFFE082
    /// <div style="padding: 10px; background-color: #FFE082"></div>
    pub const fn c200() -> u32 { 0xFFE082 }
    /// Amber\[300\] -> 0xFFD54F
    /// <div style="padding: 10px; background-color: #FFD54F"></div>
    pub const fn c300() -> u32 { 0xFFD54F }
    /// Amber\[400\] -> 0xFFCA28
    /// <div style="padding: 10px; background-color: #FFCA28"></div>
    pub const fn c400() -> u32 { 0xFFCA28 }
    /// Amber\[500\] -> 0xFFC107
    /// <div style="padding: 10px; background-color: #FFC107"></div>
    pub const fn c500() -> u32 { 0xFFC107 }
    /// Amber\[600\] -> 0xFFB300
    /// <div style="padding: 10px; background-color: #FFB300"></div>
    pub const fn c600() -> u32 { 0xFFB300 }
    /// Amber\[700\] -> 0xFFA000
    /// <div style="padding: 10px; background-color: #FFA000"></div>
    pub const fn c700() -> u32 { 0xFFA000 }
    /// Amber\[800\] -> 0xFF8F00
    /// <div style="padding: 10px; background-color: #FF8F00"></div>
    pub const fn c800() -> u32 { 0xFF8F00 }
    /// Amber\[900\] -> 0xFF6F00
    /// <div style="padding: 10px; background-color: #FF6F00"></div>
    pub const fn c900() -> u32 { 0xFF6F00 }
    /// Amber\[A100\] -> 0xFFE57F
    /// <div style="padding: 10px; background-color: #FFE57F"></div>
    pub const fn a100() -> u32 { 0xFFE57F }
    /// Amber\[A200\] -> 0xFFD740
    /// <div style="padding: 10px; background-color: #FFD740"></div>
    pub const fn a200() -> u32 { 0xFFD740 }
    /// Amber\[A400\] -> 0xFFC400
    /// <div style="padding: 10px; background-color: #FFC400"></div>
    pub const fn a400() -> u32 { 0xFFC400 }
    /// Amber\[A700\] -> 0xFFAB00
    /// <div style="padding: 10px; background-color: #FFAB00"></div>
    pub const fn a700() -> u32 { 0xFFAB00 }
}

/// Orange color and its accents
pub struct MatColorOrange;

#[rustfmt::skip]
impl MatColorOrange {
    /// Orange\[50\] -> 0xFFF3E0
    /// <div style="padding: 10px; background-color: #FFF3E0"></div>
    pub const fn c50() -> u32 { 0xFFF3E0 }
    /// Orange\[100\] -> 0xFFE0B2
    /// <div style="padding: 10px; background-color: #FFE0B2"></div>
    pub const fn c100() -> u32 { 0xFFE0B2 }
    /// Orange\[200\] -> 0xFFCC80
    /// <div style="padding: 10px; background-color: #FFCC80"></div>
    pub const fn c200() -> u32 { 0xFFCC80 }
    /// Orange\[300\] -> 0xFFB74D
    /// <div style="padding: 10px; background-color: #FFB74D"></div>
    pub const fn c300() -> u32 { 0xFFB74D }
    /// Orange\[400\] -> 0xFFA726
    /// <div style="padding: 10px; background-color: #FFA726"></div>
    pub const fn c400() -> u32 { 0xFFA726 }
    /// Orange\[500\] -> 0xFF9800
    /// <div style="padding: 10px; background-color: #FF9800"></div>
    pub const fn c500() -> u32 { 0xFF9800 }
    /// Orange\[600\] -> 0xFB8C00
    /// <div style="padding: 10px; background-color: #FB8C00"></div>
    pub const fn c600() -> u32 { 0xFB8C00 }
    /// Orange\[700\] -> 0xF57C00
    /// <div style="padding: 10px; background-color: #F57C00"></div>
    pub const fn c700() -> u32 { 0xF57C00 }
    /// Orange\[800\] -> 0xEF6C00
    /// <div style="padding: 10px; background-color: #EF6C00"></div>
    pub const fn c800() -> u32 { 0xEF6C00 }
    /// Orange\[900\] -> 0xE65100
    /// <div style="padding: 10px; background-color: #E65100"></div>
    pub const fn c900() -> u32 { 0xE65100 }
    /// Orange\[A100\] -> 0xFFD180
    /// <div style="padding: 10px; background-color: #FFD180"></div>
    pub const fn a100() -> u32 { 0xFFD180 }
    /// Orange\[A200\] -> 0xFFAB40
    /// <div style="padding: 10px; background-color: #FFAB40"></div>
    pub const fn a200() -> u32 { 0xFFAB40 }
    /// Orange\[A400\] -> 0xFF9100
    /// <div style="padding: 10px; background-color: #FF9100"></div>
    pub const fn a400() -> u32 { 0xFF9100 }
    /// Orange\[A700\] -> 0xFF6D00
    /// <div style="padding: 10px; background-color: #FF6D00"></div>
    pub const fn a700() -> u32 { 0xFF6D00 }
}

/// Deep Orange color and its accents
pub struct MatColorDeepOrange;

#[rustfmt::skip]
impl MatColorDeepOrange {
    /// DeepOrange\[50\] -> 0xFBE9E7
    /// <div style="padding: 10px; background-color: #FBE9E7"></div>
    pub const fn c50() -> u32 { 0xFBE9E7 }
    /// DeepOrange\[100\] -> 0xFFCCBC
    /// <div style="padding: 10px; background-color: #FFCCBC"></div>
    pub const fn c100() -> u32 { 0xFFCCBC }
    /// DeepOrange\[200\] -> 0xFFAB91
    /// <div style="padding: 10px; background-color: #FFAB91"></div>
    pub const fn c200() -> u32 { 0xFFAB91 }
    /// DeepOrange\[300\] -> 0xFF8A65
    /// <div style="padding: 10px; background-color: #FF8A65"></div>
    pub const fn c300() -> u32 { 0xFF8A65 }
    /// DeepOrange\[400\] -> 0xFF7043
    /// <div style="padding: 10px; background-color: #FF7043"></div>
    pub const fn c400() -> u32 { 0xFF7043 }
    /// DeepOrange\[500\] -> 0xFF5722
    /// <div style="padding: 10px; background-color: #FF5722"></div>
    pub const fn c500() -> u32 { 0xFF5722 }
    /// DeepOrange\[600\] -> 0xF4511E
    /// <div style="padding: 10px; background-color: #F4511E"></div>
    pub const fn c600() -> u32 { 0xF4511E }
    /// DeepOrange\[700\] -> 0xE64A19
    /// <div style="padding: 10px; background-color: #E64A19"></div>
    pub const fn c700() -> u32 { 0xE64A19 }
    /// DeepOrange\[800\] -> 0xD84315
    /// <div style="padding: 10px; background-color: #D84315"></div>
    pub const fn c800() -> u32 { 0xD84315 }
    /// DeepOrange\[900\] -> 0xBF360C
    /// <div style="padding: 10px; background-color: #BF360C"></div>
    pub const fn c900() -> u32 { 0xBF360C }
    /// DeepOrange\[A100\] -> 0xFF9E80
    /// <div style="padding: 10px; background-color: #FF9E80"></div>
    pub const fn a100() -> u32 { 0xFF9E80 }
    /// DeepOrange\[A200\] -> 0xFF6E40
    /// <div style="padding: 10px; background-color: #FF6E40"></div>
    pub const fn a200() -> u32 { 0xFF6E40 }
    /// DeepOrange\[A400\] -> 0xFF3D00
    /// <div style="padding: 10px; background-color: #FF3D00"></div>
    pub const fn a400() -> u32 { 0xFF3D00 }
    /// DeepOrange\[A700\] -> 0xDD2C00
    /// <div style="padding: 10px; background-color: #DD2C00"></div>
    pub const fn a700() -> u32 { 0xDD2C00 }
}

impl MatColor {
    /// Creates a color from a [`MatColor`] and an accent [`MatColorAccent`] and
    /// checks if its combination is valid. For example, not every color has
    /// every type of accent. Prefer using the `const fn` variants.
    pub fn new(_color: MatColorVariant, _accent: MatColorAccent) -> Result<u32, String> {
        todo!()
    }

    /// Access to [`MatColorRed`]
    pub const fn red() -> MatColorRed {
        MatColorRed
    }
    /// Access to [`MatColorPink`]
    pub const fn pink() -> MatColorPink {
        MatColorPink
    }
    /// Access to [`MatColorPurple`]
    pub const fn purple() -> MatColorPurple {
        MatColorPurple
    }
    /// Access to [`MatColorDeepPurple`]
    pub const fn deep_purple() -> MatColorDeepPurple {
        MatColorDeepPurple
    }
    /// Access to [`MatColorIndigo`]
    pub const fn indigo() -> MatColorIndigo {
        MatColorIndigo
    }
    /// Access to [`MatColorBlue`]
    pub const fn blue() -> MatColorBlue {
        MatColorBlue
    }
    /// Access to [`MatColorLightBlue`]
    pub const fn light_blue() -> MatColorLightBlue {
        MatColorLightBlue
    }
    /// Access to [`MatColorCyan`]
    pub const fn cyan() -> MatColorCyan {
        MatColorCyan
    }
    /// Access to [`MatColorTeal`]
    pub const fn teal() -> MatColorTeal {
        MatColorTeal
    }
    /// Access to [`MatColorGreen`]
    pub const fn green() -> MatColorGreen {
        MatColorGreen
    }
    /// Access to [`MatColorLightGreen`]
    pub const fn light_green() -> MatColorLightGreen {
        MatColorLightGreen
    }
    /// Access to [`MatColorLime`]
    pub const fn lime() -> MatColorLime {
        MatColorLime
    }
    /// Access to [`MatColorYellow`]
    pub const fn yellow() -> MatColorYellow {
        MatColorYellow
    }
    /// Access to [`MatColorAmber`]
    pub const fn amber() -> MatColorAmber {
        MatColorAmber
    }
    /// Access to [`MatColorOrange`]
    pub const fn orange() -> MatColorOrange {
        MatColorOrange
    }
    /// Access to [`MatColorDeepOrange`]
    pub const fn deep_orange() -> MatColorDeepOrange {
        MatColorDeepOrange
    }
    /// Black -> 0x000000
    pub const fn black() -> u32 {
        0x000000
    }
    /// White -> 0xFFFFFF
    pub const fn white() -> u32 {
        0xFFFFFF
    }
}
