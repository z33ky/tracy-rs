#![feature(const_cstr_unchecked, const_str_as_bytes, manually_drop_take)]
#![allow(unused)]

use std::{ffi::CStr, marker::PhantomData, mem::ManuallyDrop};

pub mod sys {
    use libc::{c_char, c_int, c_void, size_t};

    #[repr(C)]
    pub struct source_location_data {
        pub name: *const c_char,
        pub function: *const c_char,
        pub file: *const c_char,
        pub line: u32,
        pub color: u32,
    }

    #[repr(C)]
    pub struct zone_context {
        pub id: u32,
        pub active: c_int,
    }

    #[link(name="tracy")]
    extern "C" {
        pub fn ___tracy_emit_zone_begin(srcloc: *const source_location_data, active: c_int) -> zone_context;
        pub fn ___tracy_emit_zone_begin_callstack(srcloc: *const source_location_data, depth: c_int, active: c_int) -> zone_context;
        pub fn ___tracy_emit_zone_end(ctx: zone_context);
        pub fn ___tracy_emit_zone_text(ctx: zone_context, txt: *const c_char, size: size_t);
        pub fn ___tracy_emit_zone_name(ctx: zone_context, txt: *const c_char, size: size_t); 

        pub fn ___tracy_emit_frame_mark(name: *const c_char);
        pub fn ___tracy_emit_frame_mark_start(name : *const c_char);
        pub fn ___tracy_emit_frame_mark_end(name: *const c_char);
        pub fn ___tracy_emit_frame_image(image: *const c_void, w: u16, h: u16, offset: u8, flip: c_int);
    }
}

//FIXME: figure out if 'a is fine enough, or if we need 'static
pub struct SourceLocation<'a> {
    pub function: &'a CStr,
    pub file: &'a CStr,
    pub line: u32,
}

#[macro_export]
macro_rules! loc {
    () => ({
        use std::ffi::CStr;
        $crate::SourceLocation {
            //https://github.com/z33ky/tracy-rs/issues/1
            function: $crate::cstr!(module_path!(), "::fn(?)"),
            file: $crate::cstr!(file!()),
            line: line!(),
        }
    });
}

pub struct ZoneContext<'a> {
    context: ManuallyDrop<sys::zone_context>,
    marker: PhantomData<SourceLocationData<'a>>,
}

pub type CallstackDepth = libc::c_int;

impl<'a> ZoneContext<'a> {
    #[inline]
    pub fn new(loc: &SourceLocationData<'a>, active: bool) -> Self {
        Self {
            context: ManuallyDrop::new(unsafe{ sys::___tracy_emit_zone_begin(&loc.data as *const _, if active { 1 } else { 0 }) }),
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn with_callstack(loc: &SourceLocationData<'a>, depth: CallstackDepth, active: bool) -> Self {
        Self {
            context: ManuallyDrop::new(unsafe{ sys::___tracy_emit_zone_begin_callstack(&loc.data as *const _, depth, if active { 1 } else { 0 }) }),
            marker: PhantomData,
        }
    }
}

impl Drop for ZoneContext<'_> {
    #[inline]
    fn drop(&mut self) {
        //we could avoid this unsafe by manually copying (or deriving Close, though that doesn't
        //seem useful outside this specific use-case), but this is obviously safe also.
        unsafe{ sys::___tracy_emit_zone_end(ManuallyDrop::take(&mut self.context)) };
    }
}

pub type ColorType = u32;

//TODO: auto-generate via build-script
#[allow(non_upper_case_globals, clippy::unreadable_literal)]
mod colors {
    use super::ColorType;

    const Snow:                 ColorType = 0xfffafa;
    const GhostWhite:           ColorType = 0xf8f8ff;
    const WhiteSmoke:           ColorType = 0xf5f5f5;
    const Gainsboro:            ColorType = 0xdcdcdc;
    const FloralWhite:          ColorType = 0xfffaf0;
    const OldLace:              ColorType = 0xfdf5e6;
    const Linen:                ColorType = 0xfaf0e6;
    const AntiqueWhite:         ColorType = 0xfaebd7;
    const PapayaWhip:           ColorType = 0xffefd5;
    const BlanchedAlmond:       ColorType = 0xffebcd;
    const Bisque:               ColorType = 0xffe4c4;
    const PeachPuff:            ColorType = 0xffdab9;
    const NavajoWhite:          ColorType = 0xffdead;
    const Moccasin:             ColorType = 0xffe4b5;
    const Cornsilk:             ColorType = 0xfff8dc;
    const Ivory:                ColorType = 0xfffff0;
    const LemonChiffon:         ColorType = 0xfffacd;
    const Seashell:             ColorType = 0xfff5ee;
    const Honeydew:             ColorType = 0xf0fff0;
    const MintCream:            ColorType = 0xf5fffa;
    const Azure:                ColorType = 0xf0ffff;
    const AliceBlue:            ColorType = 0xf0f8ff;
    const Lavender:             ColorType = 0xe6e6fa;
    const LavenderBlush:        ColorType = 0xfff0f5;
    const MistyRose:            ColorType = 0xffe4e1;
    const White:                ColorType = 0xffffff;
    const Black:                ColorType = 0x000000;
    const DarkSlateGray:        ColorType = 0x2f4f4f;
    const DarkSlateGrey:        ColorType = 0x2f4f4f;
    const DimGray:              ColorType = 0x696969;
    const DimGrey:              ColorType = 0x696969;
    const SlateGray:            ColorType = 0x708090;
    const SlateGrey:            ColorType = 0x708090;
    const LightSlateGray:       ColorType = 0x778899;
    const LightSlateGrey:       ColorType = 0x778899;
    const Gray:                 ColorType = 0xbebebe;
    const Grey:                 ColorType = 0xbebebe;
    const X11Gray:              ColorType = 0xbebebe;
    const X11Grey:              ColorType = 0xbebebe;
    const WebGray:              ColorType = 0x808080;
    const WebGrey:              ColorType = 0x808080;
    const LightGrey:            ColorType = 0xd3d3d3;
    const LightGray:            ColorType = 0xd3d3d3;
    const MidnightBlue:         ColorType = 0x191970;
    const Navy:                 ColorType = 0x000080;
    const NavyBlue:             ColorType = 0x000080;
    const CornflowerBlue:       ColorType = 0x6495ed;
    const DarkSlateBlue:        ColorType = 0x483d8b;
    const SlateBlue:            ColorType = 0x6a5acd;
    const MediumSlateBlue:      ColorType = 0x7b68ee;
    const LightSlateBlue:       ColorType = 0x8470ff;
    const MediumBlue:           ColorType = 0x0000cd;
    const RoyalBlue:            ColorType = 0x4169e1;
    const Blue:                 ColorType = 0x0000ff;
    const DodgerBlue:           ColorType = 0x1e90ff;
    const DeepSkyBlue:          ColorType = 0x00bfff;
    const SkyBlue:              ColorType = 0x87ceeb;
    const LightSkyBlue:         ColorType = 0x87cefa;
    const SteelBlue:            ColorType = 0x4682b4;
    const LightSteelBlue:       ColorType = 0xb0c4de;
    const LightBlue:            ColorType = 0xadd8e6;
    const PowderBlue:           ColorType = 0xb0e0e6;
    const PaleTurquoise:        ColorType = 0xafeeee;
    const DarkTurquoise:        ColorType = 0x00ced1;
    const MediumTurquoise:      ColorType = 0x48d1cc;
    const Turquoise:            ColorType = 0x40e0d0;
    const Cyan:                 ColorType = 0x00ffff;
    const Aqua:                 ColorType = 0x00ffff;
    const LightCyan:            ColorType = 0xe0ffff;
    const CadetBlue:            ColorType = 0x5f9ea0;
    const MediumAquamarine:     ColorType = 0x66cdaa;
    const Aquamarine:           ColorType = 0x7fffd4;
    const DarkGreen:            ColorType = 0x006400;
    const DarkOliveGreen:       ColorType = 0x556b2f;
    const DarkSeaGreen:         ColorType = 0x8fbc8f;
    const SeaGreen:             ColorType = 0x2e8b57;
    const MediumSeaGreen:       ColorType = 0x3cb371;
    const LightSeaGreen:        ColorType = 0x20b2aa;
    const PaleGreen:            ColorType = 0x98fb98;
    const SpringGreen:          ColorType = 0x00ff7f;
    const LawnGreen:            ColorType = 0x7cfc00;
    const Green:                ColorType = 0x00ff00;
    const Lime:                 ColorType = 0x00ff00;
    const X11Green:             ColorType = 0x00ff00;
    const WebGreen:             ColorType = 0x008000;
    const Chartreuse:           ColorType = 0x7fff00;
    const MediumSpringGreen:    ColorType = 0x00fa9a;
    const GreenYellow:          ColorType = 0xadff2f;
    const LimeGreen:            ColorType = 0x32cd32;
    const YellowGreen:          ColorType = 0x9acd32;
    const ForestGreen:          ColorType = 0x228b22;
    const OliveDrab:            ColorType = 0x6b8e23;
    const DarkKhaki:            ColorType = 0xbdb76b;
    const Khaki:                ColorType = 0xf0e68c;
    const PaleGoldenrod:        ColorType = 0xeee8aa;
    const LightGoldenrodYellow: ColorType = 0xfafad2;
    const LightYellow:          ColorType = 0xffffe0;
    const Yellow:               ColorType = 0xffff00;
    const Gold:                 ColorType = 0xffd700;
    const LightGoldenrod:       ColorType = 0xeedd82;
    const Goldenrod:            ColorType = 0xdaa520;
    const DarkGoldenrod:        ColorType = 0xb8860b;
    const RosyBrown:            ColorType = 0xbc8f8f;
    const IndianRed:            ColorType = 0xcd5c5c;
    const SaddleBrown:          ColorType = 0x8b4513;
    const Sienna:               ColorType = 0xa0522d;
    const Peru:                 ColorType = 0xcd853f;
    const Burlywood:            ColorType = 0xdeb887;
    const Beige:                ColorType = 0xf5f5dc;
    const Wheat:                ColorType = 0xf5deb3;
    const SandyBrown:           ColorType = 0xf4a460;
    const Tan:                  ColorType = 0xd2b48c;
    const Chocolate:            ColorType = 0xd2691e;
    const Firebrick:            ColorType = 0xb22222;
    const Brown:                ColorType = 0xa52a2a;
    const DarkSalmon:           ColorType = 0xe9967a;
    const Salmon:               ColorType = 0xfa8072;
    const LightSalmon:          ColorType = 0xffa07a;
    const Orange:               ColorType = 0xffa500;
    const DarkOrange:           ColorType = 0xff8c00;
    const Coral:                ColorType = 0xff7f50;
    const LightCoral:           ColorType = 0xf08080;
    const Tomato:               ColorType = 0xff6347;
    const OrangeRed:            ColorType = 0xff4500;
    const Red:                  ColorType = 0xff0000;
    const HotPink:              ColorType = 0xff69b4;
    const DeepPink:             ColorType = 0xff1493;
    const Pink:                 ColorType = 0xffc0cb;
    const LightPink:            ColorType = 0xffb6c1;
    const PaleVioletRed:        ColorType = 0xdb7093;
    const Maroon:               ColorType = 0xb03060;
    const X11Maroon:            ColorType = 0xb03060;
    const WebMaroon:            ColorType = 0x800000;
    const MediumVioletRed:      ColorType = 0xc71585;
    const VioletRed:            ColorType = 0xd02090;
    const Magenta:              ColorType = 0xff00ff;
    const Fuchsia:              ColorType = 0xff00ff;
    const Violet:               ColorType = 0xee82ee;
    const Plum:                 ColorType = 0xdda0dd;
    const Orchid:               ColorType = 0xda70d6;
    const MediumOrchid:         ColorType = 0xba55d3;
    const DarkOrchid:           ColorType = 0x9932cc;
    const DarkViolet:           ColorType = 0x9400d3;
    const BlueViolet:           ColorType = 0x8a2be2;
    const Purple:               ColorType = 0xa020f0;
    const X11Purple:            ColorType = 0xa020f0;
    const WebPurple:            ColorType = 0x800080;
    const MediumPurple:         ColorType = 0x9370db;
    const Thistle:              ColorType = 0xd8bfd8;
    const Snow1:                ColorType = 0xfffafa;
    const Snow2:                ColorType = 0xeee9e9;
    const Snow3:                ColorType = 0xcdc9c9;
    const Snow4:                ColorType = 0x8b8989;
    const Seashell1:            ColorType = 0xfff5ee;
    const Seashell2:            ColorType = 0xeee5de;
    const Seashell3:            ColorType = 0xcdc5bf;
    const Seashell4:            ColorType = 0x8b8682;
    const AntiqueWhite1:        ColorType = 0xffefdb;
    const AntiqueWhite2:        ColorType = 0xeedfcc;
    const AntiqueWhite3:        ColorType = 0xcdc0b0;
    const AntiqueWhite4:        ColorType = 0x8b8378;
    const Bisque1:              ColorType = 0xffe4c4;
    const Bisque2:              ColorType = 0xeed5b7;
    const Bisque3:              ColorType = 0xcdb79e;
    const Bisque4:              ColorType = 0x8b7d6b;
    const PeachPuff1:           ColorType = 0xffdab9;
    const PeachPuff2:           ColorType = 0xeecbad;
    const PeachPuff3:           ColorType = 0xcdaf95;
    const PeachPuff4:           ColorType = 0x8b7765;
    const NavajoWhite1:         ColorType = 0xffdead;
    const NavajoWhite2:         ColorType = 0xeecfa1;
    const NavajoWhite3:         ColorType = 0xcdb38b;
    const NavajoWhite4:         ColorType = 0x8b795e;
    const LemonChiffon1:        ColorType = 0xfffacd;
    const LemonChiffon2:        ColorType = 0xeee9bf;
    const LemonChiffon3:        ColorType = 0xcdc9a5;
    const LemonChiffon4:        ColorType = 0x8b8970;
    const Cornsilk1:            ColorType = 0xfff8dc;
    const Cornsilk2:            ColorType = 0xeee8cd;
    const Cornsilk3:            ColorType = 0xcdc8b1;
    const Cornsilk4:            ColorType = 0x8b8878;
    const Ivory1:               ColorType = 0xfffff0;
    const Ivory2:               ColorType = 0xeeeee0;
    const Ivory3:               ColorType = 0xcdcdc1;
    const Ivory4:               ColorType = 0x8b8b83;
    const Honeydew1:            ColorType = 0xf0fff0;
    const Honeydew2:            ColorType = 0xe0eee0;
    const Honeydew3:            ColorType = 0xc1cdc1;
    const Honeydew4:            ColorType = 0x838b83;
    const LavenderBlush1:       ColorType = 0xfff0f5;
    const LavenderBlush2:       ColorType = 0xeee0e5;
    const LavenderBlush3:       ColorType = 0xcdc1c5;
    const LavenderBlush4:       ColorType = 0x8b8386;
    const MistyRose1:           ColorType = 0xffe4e1;
    const MistyRose2:           ColorType = 0xeed5d2;
    const MistyRose3:           ColorType = 0xcdb7b5;
    const MistyRose4:           ColorType = 0x8b7d7b;
    const Azure1:               ColorType = 0xf0ffff;
    const Azure2:               ColorType = 0xe0eeee;
    const Azure3:               ColorType = 0xc1cdcd;
    const Azure4:               ColorType = 0x838b8b;
    const SlateBlue1:           ColorType = 0x836fff;
    const SlateBlue2:           ColorType = 0x7a67ee;
    const SlateBlue3:           ColorType = 0x6959cd;
    const SlateBlue4:           ColorType = 0x473c8b;
    const RoyalBlue1:           ColorType = 0x4876ff;
    const RoyalBlue2:           ColorType = 0x436eee;
    const RoyalBlue3:           ColorType = 0x3a5fcd;
    const RoyalBlue4:           ColorType = 0x27408b;
    const Blue1:                ColorType = 0x0000ff;
    const Blue2:                ColorType = 0x0000ee;
    const Blue3:                ColorType = 0x0000cd;
    const Blue4:                ColorType = 0x00008b;
    const DodgerBlue1:          ColorType = 0x1e90ff;
    const DodgerBlue2:          ColorType = 0x1c86ee;
    const DodgerBlue3:          ColorType = 0x1874cd;
    const DodgerBlue4:          ColorType = 0x104e8b;
    const SteelBlue1:           ColorType = 0x63b8ff;
    const SteelBlue2:           ColorType = 0x5cacee;
    const SteelBlue3:           ColorType = 0x4f94cd;
    const SteelBlue4:           ColorType = 0x36648b;
    const DeepSkyBlue1:         ColorType = 0x00bfff;
    const DeepSkyBlue2:         ColorType = 0x00b2ee;
    const DeepSkyBlue3:         ColorType = 0x009acd;
    const DeepSkyBlue4:         ColorType = 0x00688b;
    const SkyBlue1:             ColorType = 0x87ceff;
    const SkyBlue2:             ColorType = 0x7ec0ee;
    const SkyBlue3:             ColorType = 0x6ca6cd;
    const SkyBlue4:             ColorType = 0x4a708b;
    const LightSkyBlue1:        ColorType = 0xb0e2ff;
    const LightSkyBlue2:        ColorType = 0xa4d3ee;
    const LightSkyBlue3:        ColorType = 0x8db6cd;
    const LightSkyBlue4:        ColorType = 0x607b8b;
    const SlateGray1:           ColorType = 0xc6e2ff;
    const SlateGray2:           ColorType = 0xb9d3ee;
    const SlateGray3:           ColorType = 0x9fb6cd;
    const SlateGray4:           ColorType = 0x6c7b8b;
    const LightSteelBlue1:      ColorType = 0xcae1ff;
    const LightSteelBlue2:      ColorType = 0xbcd2ee;
    const LightSteelBlue3:      ColorType = 0xa2b5cd;
    const LightSteelBlue4:      ColorType = 0x6e7b8b;
    const LightBlue1:           ColorType = 0xbfefff;
    const LightBlue2:           ColorType = 0xb2dfee;
    const LightBlue3:           ColorType = 0x9ac0cd;
    const LightBlue4:           ColorType = 0x68838b;
    const LightCyan1:           ColorType = 0xe0ffff;
    const LightCyan2:           ColorType = 0xd1eeee;
    const LightCyan3:           ColorType = 0xb4cdcd;
    const LightCyan4:           ColorType = 0x7a8b8b;
    const PaleTurquoise1:       ColorType = 0xbbffff;
    const PaleTurquoise2:       ColorType = 0xaeeeee;
    const PaleTurquoise3:       ColorType = 0x96cdcd;
    const PaleTurquoise4:       ColorType = 0x668b8b;
    const CadetBlue1:           ColorType = 0x98f5ff;
    const CadetBlue2:           ColorType = 0x8ee5ee;
    const CadetBlue3:           ColorType = 0x7ac5cd;
    const CadetBlue4:           ColorType = 0x53868b;
    const Turquoise1:           ColorType = 0x00f5ff;
    const Turquoise2:           ColorType = 0x00e5ee;
    const Turquoise3:           ColorType = 0x00c5cd;
    const Turquoise4:           ColorType = 0x00868b;
    const Cyan1:                ColorType = 0x00ffff;
    const Cyan2:                ColorType = 0x00eeee;
    const Cyan3:                ColorType = 0x00cdcd;
    const Cyan4:                ColorType = 0x008b8b;
    const DarkSlateGray1:       ColorType = 0x97ffff;
    const DarkSlateGray2:       ColorType = 0x8deeee;
    const DarkSlateGray3:       ColorType = 0x79cdcd;
    const DarkSlateGray4:       ColorType = 0x528b8b;
    const Aquamarine1:          ColorType = 0x7fffd4;
    const Aquamarine2:          ColorType = 0x76eec6;
    const Aquamarine3:          ColorType = 0x66cdaa;
    const Aquamarine4:          ColorType = 0x458b74;
    const DarkSeaGreen1:        ColorType = 0xc1ffc1;
    const DarkSeaGreen2:        ColorType = 0xb4eeb4;
    const DarkSeaGreen3:        ColorType = 0x9bcd9b;
    const DarkSeaGreen4:        ColorType = 0x698b69;
    const SeaGreen1:            ColorType = 0x54ff9f;
    const SeaGreen2:            ColorType = 0x4eee94;
    const SeaGreen3:            ColorType = 0x43cd80;
    const SeaGreen4:            ColorType = 0x2e8b57;
    const PaleGreen1:           ColorType = 0x9aff9a;
    const PaleGreen2:           ColorType = 0x90ee90;
    const PaleGreen3:           ColorType = 0x7ccd7c;
    const PaleGreen4:           ColorType = 0x548b54;
    const SpringGreen1:         ColorType = 0x00ff7f;
    const SpringGreen2:         ColorType = 0x00ee76;
    const SpringGreen3:         ColorType = 0x00cd66;
    const SpringGreen4:         ColorType = 0x008b45;
    const Green1:               ColorType = 0x00ff00;
    const Green2:               ColorType = 0x00ee00;
    const Green3:               ColorType = 0x00cd00;
    const Green4:               ColorType = 0x008b00;
    const Chartreuse1:          ColorType = 0x7fff00;
    const Chartreuse2:          ColorType = 0x76ee00;
    const Chartreuse3:          ColorType = 0x66cd00;
    const Chartreuse4:          ColorType = 0x458b00;
    const OliveDrab1:           ColorType = 0xc0ff3e;
    const OliveDrab2:           ColorType = 0xb3ee3a;
    const OliveDrab3:           ColorType = 0x9acd32;
    const OliveDrab4:           ColorType = 0x698b22;
    const DarkOliveGreen1:      ColorType = 0xcaff70;
    const DarkOliveGreen2:      ColorType = 0xbcee68;
    const DarkOliveGreen3:      ColorType = 0xa2cd5a;
    const DarkOliveGreen4:      ColorType = 0x6e8b3d;
    const Khaki1:               ColorType = 0xfff68f;
    const Khaki2:               ColorType = 0xeee685;
    const Khaki3:               ColorType = 0xcdc673;
    const Khaki4:               ColorType = 0x8b864e;
    const LightGoldenrod1:      ColorType = 0xffec8b;
    const LightGoldenrod2:      ColorType = 0xeedc82;
    const LightGoldenrod3:      ColorType = 0xcdbe70;
    const LightGoldenrod4:      ColorType = 0x8b814c;
    const LightYellow1:         ColorType = 0xffffe0;
    const LightYellow2:         ColorType = 0xeeeed1;
    const LightYellow3:         ColorType = 0xcdcdb4;
    const LightYellow4:         ColorType = 0x8b8b7a;
    const Yellow1:              ColorType = 0xffff00;
    const Yellow2:              ColorType = 0xeeee00;
    const Yellow3:              ColorType = 0xcdcd00;
    const Yellow4:              ColorType = 0x8b8b00;
    const Gold1:                ColorType = 0xffd700;
    const Gold2:                ColorType = 0xeec900;
    const Gold3:                ColorType = 0xcdad00;
    const Gold4:                ColorType = 0x8b7500;
    const Goldenrod1:           ColorType = 0xffc125;
    const Goldenrod2:           ColorType = 0xeeb422;
    const Goldenrod3:           ColorType = 0xcd9b1d;
    const Goldenrod4:           ColorType = 0x8b6914;
    const DarkGoldenrod1:       ColorType = 0xffb90f;
    const DarkGoldenrod2:       ColorType = 0xeead0e;
    const DarkGoldenrod3:       ColorType = 0xcd950c;
    const DarkGoldenrod4:       ColorType = 0x8b6508;
    const RosyBrown1:           ColorType = 0xffc1c1;
    const RosyBrown2:           ColorType = 0xeeb4b4;
    const RosyBrown3:           ColorType = 0xcd9b9b;
    const RosyBrown4:           ColorType = 0x8b6969;
    const IndianRed1:           ColorType = 0xff6a6a;
    const IndianRed2:           ColorType = 0xee6363;
    const IndianRed3:           ColorType = 0xcd5555;
    const IndianRed4:           ColorType = 0x8b3a3a;
    const Sienna1:              ColorType = 0xff8247;
    const Sienna2:              ColorType = 0xee7942;
    const Sienna3:              ColorType = 0xcd6839;
    const Sienna4:              ColorType = 0x8b4726;
    const Burlywood1:           ColorType = 0xffd39b;
    const Burlywood2:           ColorType = 0xeec591;
    const Burlywood3:           ColorType = 0xcdaa7d;
    const Burlywood4:           ColorType = 0x8b7355;
    const Wheat1:               ColorType = 0xffe7ba;
    const Wheat2:               ColorType = 0xeed8ae;
    const Wheat3:               ColorType = 0xcdba96;
    const Wheat4:               ColorType = 0x8b7e66;
    const Tan1:                 ColorType = 0xffa54f;
    const Tan2:                 ColorType = 0xee9a49;
    const Tan3:                 ColorType = 0xcd853f;
    const Tan4:                 ColorType = 0x8b5a2b;
    const Chocolate1:           ColorType = 0xff7f24;
    const Chocolate2:           ColorType = 0xee7621;
    const Chocolate3:           ColorType = 0xcd661d;
    const Chocolate4:           ColorType = 0x8b4513;
    const Firebrick1:           ColorType = 0xff3030;
    const Firebrick2:           ColorType = 0xee2c2c;
    const Firebrick3:           ColorType = 0xcd2626;
    const Firebrick4:           ColorType = 0x8b1a1a;
    const Brown1:               ColorType = 0xff4040;
    const Brown2:               ColorType = 0xee3b3b;
    const Brown3:               ColorType = 0xcd3333;
    const Brown4:               ColorType = 0x8b2323;
    const Salmon1:              ColorType = 0xff8c69;
    const Salmon2:              ColorType = 0xee8262;
    const Salmon3:              ColorType = 0xcd7054;
    const Salmon4:              ColorType = 0x8b4c39;
    const LightSalmon1:         ColorType = 0xffa07a;
    const LightSalmon2:         ColorType = 0xee9572;
    const LightSalmon3:         ColorType = 0xcd8162;
    const LightSalmon4:         ColorType = 0x8b5742;
    const Orange1:              ColorType = 0xffa500;
    const Orange2:              ColorType = 0xee9a00;
    const Orange3:              ColorType = 0xcd8500;
    const Orange4:              ColorType = 0x8b5a00;
    const DarkOrange1:          ColorType = 0xff7f00;
    const DarkOrange2:          ColorType = 0xee7600;
    const DarkOrange3:          ColorType = 0xcd6600;
    const DarkOrange4:          ColorType = 0x8b4500;
    const Coral1:               ColorType = 0xff7256;
    const Coral2:               ColorType = 0xee6a50;
    const Coral3:               ColorType = 0xcd5b45;
    const Coral4:               ColorType = 0x8b3e2f;
    const Tomato1:              ColorType = 0xff6347;
    const Tomato2:              ColorType = 0xee5c42;
    const Tomato3:              ColorType = 0xcd4f39;
    const Tomato4:              ColorType = 0x8b3626;
    const OrangeRed1:           ColorType = 0xff4500;
    const OrangeRed2:           ColorType = 0xee4000;
    const OrangeRed3:           ColorType = 0xcd3700;
    const OrangeRed4:           ColorType = 0x8b2500;
    const Red1:                 ColorType = 0xff0000;
    const Red2:                 ColorType = 0xee0000;
    const Red3:                 ColorType = 0xcd0000;
    const Red4:                 ColorType = 0x8b0000;
    const DeepPink1:            ColorType = 0xff1493;
    const DeepPink2:            ColorType = 0xee1289;
    const DeepPink3:            ColorType = 0xcd1076;
    const DeepPink4:            ColorType = 0x8b0a50;
    const HotPink1:             ColorType = 0xff6eb4;
    const HotPink2:             ColorType = 0xee6aa7;
    const HotPink3:             ColorType = 0xcd6090;
    const HotPink4:             ColorType = 0x8b3a62;
    const Pink1:                ColorType = 0xffb5c5;
    const Pink2:                ColorType = 0xeea9b8;
    const Pink3:                ColorType = 0xcd919e;
    const Pink4:                ColorType = 0x8b636c;
    const LightPink1:           ColorType = 0xffaeb9;
    const LightPink2:           ColorType = 0xeea2ad;
    const LightPink3:           ColorType = 0xcd8c95;
    const LightPink4:           ColorType = 0x8b5f65;
    const PaleVioletRed1:       ColorType = 0xff82ab;
    const PaleVioletRed2:       ColorType = 0xee799f;
    const PaleVioletRed3:       ColorType = 0xcd6889;
    const PaleVioletRed4:       ColorType = 0x8b475d;
    const Maroon1:              ColorType = 0xff34b3;
    const Maroon2:              ColorType = 0xee30a7;
    const Maroon3:              ColorType = 0xcd2990;
    const Maroon4:              ColorType = 0x8b1c62;
    const VioletRed1:           ColorType = 0xff3e96;
    const VioletRed2:           ColorType = 0xee3a8c;
    const VioletRed3:           ColorType = 0xcd3278;
    const VioletRed4:           ColorType = 0x8b2252;
    const Magenta1:             ColorType = 0xff00ff;
    const Magenta2:             ColorType = 0xee00ee;
    const Magenta3:             ColorType = 0xcd00cd;
    const Magenta4:             ColorType = 0x8b008b;
    const Orchid1:              ColorType = 0xff83fa;
    const Orchid2:              ColorType = 0xee7ae9;
    const Orchid3:              ColorType = 0xcd69c9;
    const Orchid4:              ColorType = 0x8b4789;
    const Plum1:                ColorType = 0xffbbff;
    const Plum2:                ColorType = 0xeeaeee;
    const Plum3:                ColorType = 0xcd96cd;
    const Plum4:                ColorType = 0x8b668b;
    const MediumOrchid1:        ColorType = 0xe066ff;
    const MediumOrchid2:        ColorType = 0xd15fee;
    const MediumOrchid3:        ColorType = 0xb452cd;
    const MediumOrchid4:        ColorType = 0x7a378b;
    const DarkOrchid1:          ColorType = 0xbf3eff;
    const DarkOrchid2:          ColorType = 0xb23aee;
    const DarkOrchid3:          ColorType = 0x9a32cd;
    const DarkOrchid4:          ColorType = 0x68228b;
    const Purple1:              ColorType = 0x9b30ff;
    const Purple2:              ColorType = 0x912cee;
    const Purple3:              ColorType = 0x7d26cd;
    const Purple4:              ColorType = 0x551a8b;
    const MediumPurple1:        ColorType = 0xab82ff;
    const MediumPurple2:        ColorType = 0x9f79ee;
    const MediumPurple3:        ColorType = 0x8968cd;
    const MediumPurple4:        ColorType = 0x5d478b;
    const Thistle1:             ColorType = 0xffe1ff;
    const Thistle2:             ColorType = 0xeed2ee;
    const Thistle3:             ColorType = 0xcdb5cd;
    const Thistle4:             ColorType = 0x8b7b8b;
    const Gray0:                ColorType = 0x000000;
    const Grey0:                ColorType = 0x000000;
    const Gray1:                ColorType = 0x030303;
    const Grey1:                ColorType = 0x030303;
    const Gray2:                ColorType = 0x050505;
    const Grey2:                ColorType = 0x050505;
    const Gray3:                ColorType = 0x080808;
    const Grey3:                ColorType = 0x080808;
    const Gray4:                ColorType = 0x0a0a0a;
    const Grey4:                ColorType = 0x0a0a0a;
    const Gray5:                ColorType = 0x0d0d0d;
    const Grey5:                ColorType = 0x0d0d0d;
    const Gray6:                ColorType = 0x0f0f0f;
    const Grey6:                ColorType = 0x0f0f0f;
    const Gray7:                ColorType = 0x121212;
    const Grey7:                ColorType = 0x121212;
    const Gray8:                ColorType = 0x141414;
    const Grey8:                ColorType = 0x141414;
    const Gray9:                ColorType = 0x171717;
    const Grey9:                ColorType = 0x171717;
    const Gray10:               ColorType = 0x1a1a1a;
    const Grey10:               ColorType = 0x1a1a1a;
    const Gray11:               ColorType = 0x1c1c1c;
    const Grey11:               ColorType = 0x1c1c1c;
    const Gray12:               ColorType = 0x1f1f1f;
    const Grey12:               ColorType = 0x1f1f1f;
    const Gray13:               ColorType = 0x212121;
    const Grey13:               ColorType = 0x212121;
    const Gray14:               ColorType = 0x242424;
    const Grey14:               ColorType = 0x242424;
    const Gray15:               ColorType = 0x262626;
    const Grey15:               ColorType = 0x262626;
    const Gray16:               ColorType = 0x292929;
    const Grey16:               ColorType = 0x292929;
    const Gray17:               ColorType = 0x2b2b2b;
    const Grey17:               ColorType = 0x2b2b2b;
    const Gray18:               ColorType = 0x2e2e2e;
    const Grey18:               ColorType = 0x2e2e2e;
    const Gray19:               ColorType = 0x303030;
    const Grey19:               ColorType = 0x303030;
    const Gray20:               ColorType = 0x333333;
    const Grey20:               ColorType = 0x333333;
    const Gray21:               ColorType = 0x363636;
    const Grey21:               ColorType = 0x363636;
    const Gray22:               ColorType = 0x383838;
    const Grey22:               ColorType = 0x383838;
    const Gray23:               ColorType = 0x3b3b3b;
    const Grey23:               ColorType = 0x3b3b3b;
    const Gray24:               ColorType = 0x3d3d3d;
    const Grey24:               ColorType = 0x3d3d3d;
    const Gray25:               ColorType = 0x404040;
    const Grey25:               ColorType = 0x404040;
    const Gray26:               ColorType = 0x424242;
    const Grey26:               ColorType = 0x424242;
    const Gray27:               ColorType = 0x454545;
    const Grey27:               ColorType = 0x454545;
    const Gray28:               ColorType = 0x474747;
    const Grey28:               ColorType = 0x474747;
    const Gray29:               ColorType = 0x4a4a4a;
    const Grey29:               ColorType = 0x4a4a4a;
    const Gray30:               ColorType = 0x4d4d4d;
    const Grey30:               ColorType = 0x4d4d4d;
    const Gray31:               ColorType = 0x4f4f4f;
    const Grey31:               ColorType = 0x4f4f4f;
    const Gray32:               ColorType = 0x525252;
    const Grey32:               ColorType = 0x525252;
    const Gray33:               ColorType = 0x545454;
    const Grey33:               ColorType = 0x545454;
    const Gray34:               ColorType = 0x575757;
    const Grey34:               ColorType = 0x575757;
    const Gray35:               ColorType = 0x595959;
    const Grey35:               ColorType = 0x595959;
    const Gray36:               ColorType = 0x5c5c5c;
    const Grey36:               ColorType = 0x5c5c5c;
    const Gray37:               ColorType = 0x5e5e5e;
    const Grey37:               ColorType = 0x5e5e5e;
    const Gray38:               ColorType = 0x616161;
    const Grey38:               ColorType = 0x616161;
    const Gray39:               ColorType = 0x636363;
    const Grey39:               ColorType = 0x636363;
    const Gray40:               ColorType = 0x666666;
    const Grey40:               ColorType = 0x666666;
    const Gray41:               ColorType = 0x696969;
    const Grey41:               ColorType = 0x696969;
    const Gray42:               ColorType = 0x6b6b6b;
    const Grey42:               ColorType = 0x6b6b6b;
    const Gray43:               ColorType = 0x6e6e6e;
    const Grey43:               ColorType = 0x6e6e6e;
    const Gray44:               ColorType = 0x707070;
    const Grey44:               ColorType = 0x707070;
    const Gray45:               ColorType = 0x737373;
    const Grey45:               ColorType = 0x737373;
    const Gray46:               ColorType = 0x757575;
    const Grey46:               ColorType = 0x757575;
    const Gray47:               ColorType = 0x787878;
    const Grey47:               ColorType = 0x787878;
    const Gray48:               ColorType = 0x7a7a7a;
    const Grey48:               ColorType = 0x7a7a7a;
    const Gray49:               ColorType = 0x7d7d7d;
    const Grey49:               ColorType = 0x7d7d7d;
    const Gray50:               ColorType = 0x7f7f7f;
    const Grey50:               ColorType = 0x7f7f7f;
    const Gray51:               ColorType = 0x828282;
    const Grey51:               ColorType = 0x828282;
    const Gray52:               ColorType = 0x858585;
    const Grey52:               ColorType = 0x858585;
    const Gray53:               ColorType = 0x878787;
    const Grey53:               ColorType = 0x878787;
    const Gray54:               ColorType = 0x8a8a8a;
    const Grey54:               ColorType = 0x8a8a8a;
    const Gray55:               ColorType = 0x8c8c8c;
    const Grey55:               ColorType = 0x8c8c8c;
    const Gray56:               ColorType = 0x8f8f8f;
    const Grey56:               ColorType = 0x8f8f8f;
    const Gray57:               ColorType = 0x919191;
    const Grey57:               ColorType = 0x919191;
    const Gray58:               ColorType = 0x949494;
    const Grey58:               ColorType = 0x949494;
    const Gray59:               ColorType = 0x969696;
    const Grey59:               ColorType = 0x969696;
    const Gray60:               ColorType = 0x999999;
    const Grey60:               ColorType = 0x999999;
    const Gray61:               ColorType = 0x9c9c9c;
    const Grey61:               ColorType = 0x9c9c9c;
    const Gray62:               ColorType = 0x9e9e9e;
    const Grey62:               ColorType = 0x9e9e9e;
    const Gray63:               ColorType = 0xa1a1a1;
    const Grey63:               ColorType = 0xa1a1a1;
    const Gray64:               ColorType = 0xa3a3a3;
    const Grey64:               ColorType = 0xa3a3a3;
    const Gray65:               ColorType = 0xa6a6a6;
    const Grey65:               ColorType = 0xa6a6a6;
    const Gray66:               ColorType = 0xa8a8a8;
    const Grey66:               ColorType = 0xa8a8a8;
    const Gray67:               ColorType = 0xababab;
    const Grey67:               ColorType = 0xababab;
    const Gray68:               ColorType = 0xadadad;
    const Grey68:               ColorType = 0xadadad;
    const Gray69:               ColorType = 0xb0b0b0;
    const Grey69:               ColorType = 0xb0b0b0;
    const Gray70:               ColorType = 0xb3b3b3;
    const Grey70:               ColorType = 0xb3b3b3;
    const Gray71:               ColorType = 0xb5b5b5;
    const Grey71:               ColorType = 0xb5b5b5;
    const Gray72:               ColorType = 0xb8b8b8;
    const Grey72:               ColorType = 0xb8b8b8;
    const Gray73:               ColorType = 0xbababa;
    const Grey73:               ColorType = 0xbababa;
    const Gray74:               ColorType = 0xbdbdbd;
    const Grey74:               ColorType = 0xbdbdbd;
    const Gray75:               ColorType = 0xbfbfbf;
    const Grey75:               ColorType = 0xbfbfbf;
    const Gray76:               ColorType = 0xc2c2c2;
    const Grey76:               ColorType = 0xc2c2c2;
    const Gray77:               ColorType = 0xc4c4c4;
    const Grey77:               ColorType = 0xc4c4c4;
    const Gray78:               ColorType = 0xc7c7c7;
    const Grey78:               ColorType = 0xc7c7c7;
    const Gray79:               ColorType = 0xc9c9c9;
    const Grey79:               ColorType = 0xc9c9c9;
    const Gray80:               ColorType = 0xcccccc;
    const Grey80:               ColorType = 0xcccccc;
    const Gray81:               ColorType = 0xcfcfcf;
    const Grey81:               ColorType = 0xcfcfcf;
    const Gray82:               ColorType = 0xd1d1d1;
    const Grey82:               ColorType = 0xd1d1d1;
    const Gray83:               ColorType = 0xd4d4d4;
    const Grey83:               ColorType = 0xd4d4d4;
    const Gray84:               ColorType = 0xd6d6d6;
    const Grey84:               ColorType = 0xd6d6d6;
    const Gray85:               ColorType = 0xd9d9d9;
    const Grey85:               ColorType = 0xd9d9d9;
    const Gray86:               ColorType = 0xdbdbdb;
    const Grey86:               ColorType = 0xdbdbdb;
    const Gray87:               ColorType = 0xdedede;
    const Grey87:               ColorType = 0xdedede;
    const Gray88:               ColorType = 0xe0e0e0;
    const Grey88:               ColorType = 0xe0e0e0;
    const Gray89:               ColorType = 0xe3e3e3;
    const Grey89:               ColorType = 0xe3e3e3;
    const Gray90:               ColorType = 0xe5e5e5;
    const Grey90:               ColorType = 0xe5e5e5;
    const Gray91:               ColorType = 0xe8e8e8;
    const Grey91:               ColorType = 0xe8e8e8;
    const Gray92:               ColorType = 0xebebeb;
    const Grey92:               ColorType = 0xebebeb;
    const Gray93:               ColorType = 0xededed;
    const Grey93:               ColorType = 0xededed;
    const Gray94:               ColorType = 0xf0f0f0;
    const Grey94:               ColorType = 0xf0f0f0;
    const Gray95:               ColorType = 0xf2f2f2;
    const Grey95:               ColorType = 0xf2f2f2;
    const Gray96:               ColorType = 0xf5f5f5;
    const Grey96:               ColorType = 0xf5f5f5;
    const Gray97:               ColorType = 0xf7f7f7;
    const Grey97:               ColorType = 0xf7f7f7;
    const Gray98:               ColorType = 0xfafafa;
    const Grey98:               ColorType = 0xfafafa;
    const Gray99:               ColorType = 0xfcfcfc;
    const Grey99:               ColorType = 0xfcfcfc;
    const Gray100:              ColorType = 0xffffff;
    const Grey100:              ColorType = 0xffffff;
    const DarkGrey:             ColorType = 0xa9a9a9;
    const DarkGray:             ColorType = 0xa9a9a9;
    const DarkBlue:             ColorType = 0x00008b;
    const DarkCyan:             ColorType = 0x008b8b;
    const DarkMagenta:          ColorType = 0x8b008b;
    const DarkRed:              ColorType = 0x8b0000;
    const LightGreen:           ColorType = 0x90ee90;
    const Crimson:              ColorType = 0xdc143c;
    const Indigo:               ColorType = 0x4b0082;
    const Olive:                ColorType = 0x808000;
    const RebeccaPurple:        ColorType = 0x663399;
    const Silver:               ColorType = 0xc0c0c0;
    const Teal:                 ColorType = 0x008080;
}

pub struct SourceLocationData<'a> {
    data: sys::source_location_data,
    marker: PhantomData<&'a CStr>
}

impl<'a> SourceLocationData<'a> {
    #[inline]
    pub const fn with_name_and_color(loc: &SourceLocation<'a>, name: &'a CStr, color: ColorType) -> Self {
        Self {
            data: sys::source_location_data {
                name: name.as_ptr(),
                function: loc.function.as_ptr(),
                file: loc.file.as_ptr(),
                line: loc.line,
                color,
            },
            marker: PhantomData,
        }
    }

    #[inline]
    pub const fn with_color(loc: &SourceLocation<'a>, color: ColorType) -> Self {
        Self {
            data: sys::source_location_data {
                name: std::ptr::null(),
                function: loc.function.as_ptr(),
                file: loc.file.as_ptr(),
                line: loc.line,
                color,
            },
            marker: PhantomData,
        }
    }

    #[inline]
    pub const fn with_name(loc: &SourceLocation<'a>, name: &'a CStr) -> Self {
        Self::with_name_and_color(loc, name, 0)
    }

    #[inline]
    pub const fn without_name_or_color(loc: &SourceLocation<'a>) -> Self {
        Self::with_color(loc, 0)
    }
}

#[macro_export]
macro_rules! cstr {
    ( $( $str: expr ),* ) => (unsafe{ ::std::ffi::CStr::from_bytes_with_nul_unchecked(concat!($( $str ),* , "\0").as_bytes()) })
}

//TODO: empty on !cfg(feature = "enable")
#[macro_export]
macro_rules! zone {
    ($active: expr) => (let _zone = ZoneContext::new(&{
        const LOC: $crate::SourceLocationData = $crate::SourceLocationData::without_name_or_color(
            &$crate::loc!(),
        );
        LOC
    }, $active););
}

#[macro_export]
macro_rules! zone_n {
    ($name: expr, $active: expr) => (let _zone = $crate::ZoneContext::new(&{
        const LOC: $crate::SourceLocationData = $crate::SourceLocationData::with_name(
            &$crate::loc!(),
            $crate::cstr!($name),
        );
        LOC
    }, $active););
}

#[macro_export]
macro_rules! zone_c {
    ($color: expr, $active: expr) => (let _zone = $crate::ZoneContext::new(&{
        const LOC: $crate::SourceLocationData = $crate::SourceLocationData::with_color(
            &$crate::loc!(),
            $color,
        );
        LOC
    }, $active););
}

#[macro_export]
macro_rules! zone_nc {
    ($name: expr, $color: expr, $active: expr) => (let _zone = $crate::ZoneContext::new(&{
        const LOC: $crate::SourceLocationData = $crate::SourceLocationData::with_name_and_color(
            &$crate::loc!(),
            $crate::cstr!($name),
            $color,
        );
        LOC
    }, $active););
}

#[macro_export]
macro_rules! zone_s {
    ($depth: expr, $active: expr) => (let _zone = ZoneContext::with_callstack(&{
        const LOC: $crate::SourceLocationData = $crate::SourceLocationData::without_name_or_color(
            &$crate::loc!(),
        );
        LOC
    }, $depth, $active););
}

#[macro_export]
macro_rules! zone_ns {
    ($name: expr, $depth: expr, $active: expr) => (let _zone = $crate::ZoneContext::with_callstack(&{
        const LOC: $crate::SourceLocationData = $crate::SourceLocationData::with_name(
            &$crate::loc!(),
            $crate::cstr!($name),
        );
        LOC
    }, $depth, $active););
}

#[macro_export]
macro_rules! zone_cs {
    ($color: expr, $depth: expr, $active: expr) => (let _zone = $crate::ZoneContext::with_callstack(&{
        const LOC: $crate::SourceLocationData = $crate::SourceLocationData::with_color(
            &$crate::loc!(),
            $color,
        );
        LOC
    }, $depth, $active););
}

#[macro_export]
macro_rules! zone_ncs {
    ($name: expr, $color: expr, $depth: expr, $active: expr) => (let _zone = $crate::ZoneContext::with_callstack(&{
        const LOC: $crate::SourceLocationData = $crate::SourceLocationData::with_name_and_color(
            &$crate::loc!(),
            $crate::cstr!($name),
            $color,
        );
        LOC
    }, $depth, $active););
}

#[macro_export]
macro_rules! frame_mark {
    () => (unsafe { $crate::sys::___tracy_emit_frame_mark(::core::ptr::null()) };);
}

#[macro_export]
macro_rules! frame_mark_name {
    ($name: expr) => (unsafe { $crate::sys::___tracy_emit_frame_mark($crate::cstr!($name)) };);
}

#[macro_export]
macro_rules! frame_mark_start {
    ($name: expr) => (unsafe { $crate::sys::___tracy_emit_frame_mark_start($crate::cstr!($name)) };);
}

#[macro_export]
macro_rules! frame_mark_end {
    ($name: expr) => (unsafe { $crate::sys::___tracy_emit_frame_mark_end($crate::cstr!($name)) };);
}

//TODO: bounds-checked variant
#[macro_export]
macro_rules! frame_mark_image_unchecked {
    ($image: expr, $width: expr, $height: expr, $offset: expr, $flip: expr) => ($crate::sys::___tracy_emit_frame_mark_image($image as *const ::libc::c_void, $width, $height, $offset, if $flip { 1 } else { 0 }););
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        zone_ncs!("foo", 0, 1, true);
        ZoneContext::with_callstack(&SourceLocationData::with_name_and_color(&loc!(), cstr!("foo"), 0), 1, true);
    }
}
