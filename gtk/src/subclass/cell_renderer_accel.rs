use gdk_sys;
use gtk_sys;

use libc::{c_char, c_uint};

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::GString;

use super::cell_renderer_text::CellRendererTextImpl;
use CellRendererAccel;
use CellRendererText;

pub trait CellRendererAccelImpl: CellRendererAccelImplExt + CellRendererTextImpl {
    fn accel_edited(
        &self,
        renderer: &CellRendererAccel,
        path: &str,
        accel_key: u32,
        accel_mods: gdk::ModifierType,
        hardware_keycode: u32,
    ) {
        self.parent_accel_edited(renderer, path, accel_key, accel_mods, hardware_keycode);
    }

    fn accel_cleared(&self, renderer: &CellRendererAccel, path: &str) {
        self.parent_accel_cleared(renderer, path);
    }
}

pub trait CellRendererAccelImplExt {
    fn parent_accel_edited(
        &self,
        renderer: &CellRendererAccel,
        path: &str,
        accel_key: u32,
        accel_mods: gdk::ModifierType,
        hardware_keycode: u32,
    );
    fn parent_accel_cleared(&self, renderer: &CellRendererAccel, path: &str);
}

impl<T: CellRendererAccelImpl> CellRendererAccelImplExt for T {
    fn parent_accel_edited(
        &self,
        renderer: &CellRendererAccel,
        path: &str,
        accel_key: u32,
        accel_mods: gdk::ModifierType,
        hardware_keycode: u32,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkCellRendererAccelClass;
            if let Some(f) = (*parent_class).accel_edited {
                f(
                    renderer.to_glib_none().0,
                    path.to_glib_none().0,
                    accel_key,
                    accel_mods.to_glib(),
                    hardware_keycode,
                )
            }
        }
    }

    fn parent_accel_cleared(&self, renderer: &CellRendererAccel, path: &str) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkCellRendererAccelClass;
            if let Some(f) = (*parent_class).accel_cleared {
                f(renderer.to_glib_none().0, path.to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: CellRendererAccelImpl> IsSubclassable<T> for CellRendererAccel {
    fn override_vfuncs(class: &mut ::glib::Class<Self>) {
        <CellRendererText as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();
        klass.accel_edited = Some(cell_renderer_accel_edited::<T>);
        klass.accel_cleared = Some(cell_renderer_accel_cleared::<T>);
    }
}

unsafe extern "C" fn cell_renderer_accel_edited<T: CellRendererAccelImpl>(
    ptr: *mut gtk_sys::GtkCellRendererAccel,
    path: *const c_char,
    accel_key: c_uint,
    accel_mods: gdk_sys::GdkModifierType,
    hardware_keycode: c_uint,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap = from_glib_borrow(ptr);

    imp.accel_edited(
        &wrap,
        &GString::from_glib_borrow(path),
        accel_key,
        from_glib(accel_mods),
        hardware_keycode,
    )
}

unsafe extern "C" fn cell_renderer_accel_cleared<T: CellRendererAccelImpl>(
    ptr: *mut gtk_sys::GtkCellRendererAccel,
    path: *const c_char,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap = from_glib_borrow(ptr);

    imp.accel_cleared(&wrap, &GString::from_glib_borrow(path))
}
