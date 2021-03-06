// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use gtk_sys;
use std::convert::TryFrom;
use TextAttributes;
use TextIter;

impl TextIter {
    pub fn get_attributes(&self, values: &TextAttributes) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_get_attributes(
                self.to_glib_none().0,
                mut_override(values.to_glib_none().0),
            ))
        }
    }

    pub fn get_char(&self) -> Option<char> {
        let ret = unsafe { gtk_sys::gtk_text_iter_get_char(self.to_glib_none().0) };

        if ret == 0 {
            return None;
        }

        Some(TryFrom::try_from(ret).expect("conversion from an invalid Unicode value attempted"))
    }
}
