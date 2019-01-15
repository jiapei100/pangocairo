// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use cairo;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use pango;
use std::fmt;

glib_wrapper! {
    pub struct Font(Interface<ffi::PangoCairoFont>) @requires pango::Font;

    match fn {
        get_type => || ffi::pango_cairo_font_get_type(),
    }
}

pub const NONE_FONT: Option<&Font> = None;

pub trait FontExt: 'static {
    fn get_scaled_font(&self) -> Option<cairo::ScaledFont>;
}

impl<O: IsA<Font>> FontExt for O {
    fn get_scaled_font(&self) -> Option<cairo::ScaledFont> {
        unsafe {
            from_glib_full(ffi::pango_cairo_font_get_scaled_font(self.as_ref().to_glib_none().0))
        }
    }
}

impl fmt::Display for Font {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Font")
    }
}
