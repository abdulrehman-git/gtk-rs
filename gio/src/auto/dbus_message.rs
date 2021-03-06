// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use glib;
#[cfg(any(unix, feature = "dox"))]
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;
use DBusCapabilityFlags;
use DBusMessageByteOrder;
use DBusMessageFlags;
use DBusMessageHeaderField;
use DBusMessageType;
#[cfg(any(unix, feature = "dox"))]
use UnixFDList;

glib_wrapper! {
    pub struct DBusMessage(Object<gio_sys::GDBusMessage>);

    match fn {
        get_type => || gio_sys::g_dbus_message_get_type(),
    }
}

impl DBusMessage {
    pub fn new() -> DBusMessage {
        unsafe { from_glib_full(gio_sys::g_dbus_message_new()) }
    }

    pub fn from_blob(
        blob: &[u8],
        capabilities: DBusCapabilityFlags,
    ) -> Result<DBusMessage, glib::Error> {
        let blob_len = blob.len() as usize;
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_dbus_message_new_from_blob(
                blob.to_glib_none().0,
                blob_len,
                capabilities.to_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn new_method_call(
        name: Option<&str>,
        path: &str,
        interface_: Option<&str>,
        method: &str,
    ) -> DBusMessage {
        unsafe {
            from_glib_full(gio_sys::g_dbus_message_new_method_call(
                name.to_glib_none().0,
                path.to_glib_none().0,
                interface_.to_glib_none().0,
                method.to_glib_none().0,
            ))
        }
    }

    pub fn new_signal(path: &str, interface_: &str, signal: &str) -> DBusMessage {
        unsafe {
            from_glib_full(gio_sys::g_dbus_message_new_signal(
                path.to_glib_none().0,
                interface_.to_glib_none().0,
                signal.to_glib_none().0,
            ))
        }
    }

    pub fn copy(&self) -> Result<DBusMessage, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_dbus_message_copy(self.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn get_arg0(&self) -> Option<GString> {
        unsafe { from_glib_none(gio_sys::g_dbus_message_get_arg0(self.to_glib_none().0)) }
    }

    pub fn get_body(&self) -> Option<glib::Variant> {
        unsafe { from_glib_none(gio_sys::g_dbus_message_get_body(self.to_glib_none().0)) }
    }

    pub fn get_byte_order(&self) -> DBusMessageByteOrder {
        unsafe {
            from_glib(gio_sys::g_dbus_message_get_byte_order(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_destination(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gio_sys::g_dbus_message_get_destination(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_error_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gio_sys::g_dbus_message_get_error_name(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_flags(&self) -> DBusMessageFlags {
        unsafe { from_glib(gio_sys::g_dbus_message_get_flags(self.to_glib_none().0)) }
    }

    pub fn get_header(&self, header_field: DBusMessageHeaderField) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(gio_sys::g_dbus_message_get_header(
                self.to_glib_none().0,
                header_field.to_glib(),
            ))
        }
    }

    pub fn get_interface(&self) -> Option<GString> {
        unsafe { from_glib_none(gio_sys::g_dbus_message_get_interface(self.to_glib_none().0)) }
    }

    pub fn get_locked(&self) -> bool {
        unsafe { from_glib(gio_sys::g_dbus_message_get_locked(self.to_glib_none().0)) }
    }

    pub fn get_member(&self) -> Option<GString> {
        unsafe { from_glib_none(gio_sys::g_dbus_message_get_member(self.to_glib_none().0)) }
    }

    pub fn get_message_type(&self) -> DBusMessageType {
        unsafe {
            from_glib(gio_sys::g_dbus_message_get_message_type(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    pub fn get_num_unix_fds(&self) -> u32 {
        unsafe { gio_sys::g_dbus_message_get_num_unix_fds(self.to_glib_none().0) }
    }

    pub fn get_path(&self) -> Option<GString> {
        unsafe { from_glib_none(gio_sys::g_dbus_message_get_path(self.to_glib_none().0)) }
    }

    pub fn get_reply_serial(&self) -> u32 {
        unsafe { gio_sys::g_dbus_message_get_reply_serial(self.to_glib_none().0) }
    }

    pub fn get_sender(&self) -> Option<GString> {
        unsafe { from_glib_none(gio_sys::g_dbus_message_get_sender(self.to_glib_none().0)) }
    }

    pub fn get_serial(&self) -> u32 {
        unsafe { gio_sys::g_dbus_message_get_serial(self.to_glib_none().0) }
    }

    pub fn get_signature(&self) -> Option<GString> {
        unsafe { from_glib_none(gio_sys::g_dbus_message_get_signature(self.to_glib_none().0)) }
    }

    #[cfg(any(unix, feature = "dox"))]
    pub fn get_unix_fd_list(&self) -> Option<UnixFDList> {
        unsafe {
            from_glib_none(gio_sys::g_dbus_message_get_unix_fd_list(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn lock(&self) {
        unsafe {
            gio_sys::g_dbus_message_lock(self.to_glib_none().0);
        }
    }

    //pub fn new_method_error(&self, error_name: &str, error_message_format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<DBusMessage> {
    //    unsafe { TODO: call gio_sys:g_dbus_message_new_method_error() }
    //}

    pub fn new_method_error_literal(
        &self,
        error_name: &str,
        error_message: &str,
    ) -> Option<DBusMessage> {
        unsafe {
            from_glib_full(gio_sys::g_dbus_message_new_method_error_literal(
                self.to_glib_none().0,
                error_name.to_glib_none().0,
                error_message.to_glib_none().0,
            ))
        }
    }

    //pub fn new_method_error_valist(&self, error_name: &str, error_message_format: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> Option<DBusMessage> {
    //    unsafe { TODO: call gio_sys:g_dbus_message_new_method_error_valist() }
    //}

    pub fn new_method_reply(&self) -> Option<DBusMessage> {
        unsafe {
            from_glib_full(gio_sys::g_dbus_message_new_method_reply(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn print(&self, indent: u32) -> Option<GString> {
        unsafe { from_glib_full(gio_sys::g_dbus_message_print(self.to_glib_none().0, indent)) }
    }

    pub fn set_body(&self, body: &glib::Variant) {
        unsafe {
            gio_sys::g_dbus_message_set_body(self.to_glib_none().0, body.to_glib_none().0);
        }
    }

    pub fn set_byte_order(&self, byte_order: DBusMessageByteOrder) {
        unsafe {
            gio_sys::g_dbus_message_set_byte_order(self.to_glib_none().0, byte_order.to_glib());
        }
    }

    pub fn set_destination(&self, value: &str) {
        unsafe {
            gio_sys::g_dbus_message_set_destination(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_error_name(&self, value: &str) {
        unsafe {
            gio_sys::g_dbus_message_set_error_name(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_flags(&self, flags: DBusMessageFlags) {
        unsafe {
            gio_sys::g_dbus_message_set_flags(self.to_glib_none().0, flags.to_glib());
        }
    }

    pub fn set_header(&self, header_field: DBusMessageHeaderField, value: Option<&glib::Variant>) {
        unsafe {
            gio_sys::g_dbus_message_set_header(
                self.to_glib_none().0,
                header_field.to_glib(),
                value.to_glib_none().0,
            );
        }
    }

    pub fn set_interface(&self, value: &str) {
        unsafe {
            gio_sys::g_dbus_message_set_interface(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_member(&self, value: &str) {
        unsafe {
            gio_sys::g_dbus_message_set_member(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_message_type(&self, type_: DBusMessageType) {
        unsafe {
            gio_sys::g_dbus_message_set_message_type(self.to_glib_none().0, type_.to_glib());
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    pub fn set_num_unix_fds(&self, value: u32) {
        unsafe {
            gio_sys::g_dbus_message_set_num_unix_fds(self.to_glib_none().0, value);
        }
    }

    pub fn set_path(&self, value: &str) {
        unsafe {
            gio_sys::g_dbus_message_set_path(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_reply_serial(&self, value: u32) {
        unsafe {
            gio_sys::g_dbus_message_set_reply_serial(self.to_glib_none().0, value);
        }
    }

    pub fn set_sender(&self, value: &str) {
        unsafe {
            gio_sys::g_dbus_message_set_sender(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_serial(&self, serial: u32) {
        unsafe {
            gio_sys::g_dbus_message_set_serial(self.to_glib_none().0, serial);
        }
    }

    pub fn set_signature(&self, value: &str) {
        unsafe {
            gio_sys::g_dbus_message_set_signature(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    pub fn set_unix_fd_list<P: IsA<UnixFDList>>(&self, fd_list: Option<&P>) {
        unsafe {
            gio_sys::g_dbus_message_set_unix_fd_list(
                self.to_glib_none().0,
                fd_list.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    pub fn to_blob(&self, capabilities: DBusCapabilityFlags) -> Result<Vec<u8>, glib::Error> {
        unsafe {
            let mut out_size = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_dbus_message_to_blob(
                self.to_glib_none().0,
                out_size.as_mut_ptr(),
                capabilities.to_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(FromGlibContainer::from_glib_full_num(
                    ret,
                    out_size.assume_init() as usize,
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn to_gerror(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_dbus_message_to_gerror(self.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn bytes_needed(blob: &[u8]) -> Result<isize, glib::Error> {
        let blob_len = blob.len() as usize;
        unsafe {
            let mut error = ptr::null_mut();
            let ret =
                gio_sys::g_dbus_message_bytes_needed(blob.to_glib_none().0, blob_len, &mut error);
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn connect_property_locked_notify<F: Fn(&DBusMessage) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_locked_trampoline<F: Fn(&DBusMessage) + 'static>(
            this: *mut gio_sys::GDBusMessage,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::locked\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_locked_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for DBusMessage {
    fn default() -> Self {
        Self::new()
    }
}
