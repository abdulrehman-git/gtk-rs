// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::subclass::prelude::*;

use glib::translate::*;

use super::window::WindowImpl;
use Plug;
use Window;

pub trait PlugImpl: PlugImplExt + WindowImpl {
    fn embedded(&self, plug: &Plug) {
        self.parent_embedded(plug)
    }
}

pub trait PlugImplExt {
    fn parent_embedded(&self, plug: &Plug);
}

impl<T: PlugImpl> PlugImplExt for T {
    fn parent_embedded(&self, plug: &Plug) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkPlugClass;
            if let Some(f) = (*parent_class).embedded {
                f(plug.to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: PlugImpl> IsSubclassable<T> for Plug {
    fn override_vfuncs(class: &mut ::glib::Class<Self>) {
        <Window as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();
        klass.embedded = Some(plug_embedded::<T>);
    }
}

unsafe extern "C" fn plug_embedded<T: PlugImpl>(ptr: *mut gtk_sys::GtkPlug) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Plug> = from_glib_borrow(ptr);

    imp.embedded(&wrap)
}
