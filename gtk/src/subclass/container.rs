use gtk_sys;

use glib::translate::*;

use glib::subclass::prelude::*;

use super::widget::WidgetImpl;
use Container;
use Widget;
use WidgetPath;

pub trait ContainerImpl: ContainerImplExt + WidgetImpl {
    fn add(&self, container: &Container, widget: &Widget) {
        self.parent_add(container, widget)
    }

    fn remove(&self, container: &Container, widget: &Widget) {
        self.parent_remove(container, widget)
    }

    fn check_resize(&self, container: &Container) {
        self.parent_check_resize(container)
    }

    fn set_focus_child(&self, container: &Container, widget: Option<&Widget>) {
        self.parent_set_focus_child(container, widget)
    }

    fn child_type(&self, container: &Container) -> glib::Type {
        self.parent_child_type(container)
    }

    fn get_path_for_child(&self, container: &Container, widget: &Widget) -> WidgetPath {
        self.parent_get_path_for_child(container, widget)
    }

    fn forall(&self, container: &Container, include_internals: bool, callback: &Callback) {
        self.parent_forall(container, include_internals, callback);
    }
}

pub trait ContainerImplExt {
    fn parent_add(&self, container: &Container, widget: &Widget);
    fn parent_remove(&self, container: &Container, widget: &Widget);
    fn parent_check_resize(&self, container: &Container);
    fn parent_set_focus_child(&self, container: &Container, widget: Option<&Widget>);
    fn parent_child_type(&self, container: &Container) -> glib::Type;
    fn parent_get_path_for_child(&self, container: &Container, widget: &Widget) -> WidgetPath;
    fn parent_forall(&self, container: &Container, include_internals: bool, callback: &Callback);
}

impl<T: ContainerImpl> ContainerImplExt for T {
    fn parent_add(&self, container: &Container, widget: &Widget) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkContainerClass;
            if let Some(f) = (*parent_class).add {
                f(container.to_glib_none().0, widget.to_glib_none().0)
            }
        }
    }

    fn parent_remove(&self, container: &Container, widget: &Widget) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkContainerClass;
            if let Some(f) = (*parent_class).remove {
                f(container.to_glib_none().0, widget.to_glib_none().0)
            }
        }
    }

    fn parent_check_resize(&self, container: &Container) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkContainerClass;
            if let Some(f) = (*parent_class).check_resize {
                f(container.to_glib_none().0)
            }
        }
    }

    fn parent_set_focus_child(&self, container: &Container, widget: Option<&Widget>) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkContainerClass;
            if let Some(f) = (*parent_class).set_focus_child {
                f(container.to_glib_none().0, widget.to_glib_none().0)
            }
        }
    }

    fn parent_child_type(&self, container: &Container) -> glib::Type {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkContainerClass;
            if let Some(f) = (*parent_class).child_type {
                from_glib(f(container.to_glib_none().0))
            } else {
                glib::Type::Unit
            }
        }
    }

    fn parent_get_path_for_child(&self, container: &Container, widget: &Widget) -> WidgetPath {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkContainerClass;
            let f = (*parent_class)
                .get_path_for_child
                .expect("No parent class impl for \"get_path_for_child\"");
            from_glib_none(f(container.to_glib_none().0, widget.to_glib_none().0))
        }
    }

    fn parent_forall(&self, container: &Container, include_internals: bool, callback: &Callback) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkContainerClass;
            if let Some(f) = (*parent_class).forall {
                f(
                    container.to_glib_none().0,
                    include_internals.to_glib(),
                    callback.callback,
                    callback.user_data,
                )
            }
        }
    }
}

unsafe impl<T: ContainerImpl> IsSubclassable<T> for Container {
    fn override_vfuncs(class: &mut ::glib::Class<Self>) {
        <Widget as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();
        klass.add = Some(container_add::<T>);
        klass.remove = Some(container_remove::<T>);
        klass.check_resize = Some(container_check_resize::<T>);
        klass.set_focus_child = Some(container_set_focus_child::<T>);
        klass.child_type = Some(container_child_type::<T>);
        klass.get_path_for_child = Some(container_get_path_for_child::<T>);
        klass.forall = Some(container_forall::<T>);
    }
}

unsafe extern "C" fn container_add<T: ContainerImpl>(
    ptr: *mut gtk_sys::GtkContainer,
    wdgtptr: *mut gtk_sys::GtkWidget,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Container> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(wdgtptr);

    imp.add(&wrap, &widget)
}

unsafe extern "C" fn container_remove<T: ContainerImpl>(
    ptr: *mut gtk_sys::GtkContainer,
    wdgtptr: *mut gtk_sys::GtkWidget,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Container> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(wdgtptr);

    imp.remove(&wrap, &widget)
}

unsafe extern "C" fn container_check_resize<T: ContainerImpl>(ptr: *mut gtk_sys::GtkContainer) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Container> = from_glib_borrow(ptr);

    imp.check_resize(&wrap)
}

unsafe extern "C" fn container_set_focus_child<T: ContainerImpl>(
    ptr: *mut gtk_sys::GtkContainer,
    wdgtptr: *mut gtk_sys::GtkWidget,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Container> = from_glib_borrow(ptr);
    let widget: Borrowed<Option<Widget>> = from_glib_borrow(wdgtptr);

    imp.set_focus_child(&wrap, widget.as_ref().as_ref())
}

unsafe extern "C" fn container_child_type<T: ContainerImpl>(
    ptr: *mut gtk_sys::GtkContainer,
) -> glib_sys::GType {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Container> = from_glib_borrow(ptr);

    imp.child_type(&wrap).to_glib()
}

unsafe extern "C" fn container_get_path_for_child<T: ContainerImpl>(
    ptr: *mut gtk_sys::GtkContainer,
    wdgtptr: *mut gtk_sys::GtkWidget,
) -> *mut gtk_sys::GtkWidgetPath {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Container> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(wdgtptr);

    imp.get_path_for_child(&wrap, &widget).to_glib_none().0
}

unsafe extern "C" fn container_forall<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkContainer,
    include_internals: glib_sys::gboolean,
    callback: gtk_sys::GtkCallback,
    user_data: glib_sys::gpointer,
) where
    T: ContainerImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Container> = from_glib_borrow(ptr);
    let callback = Callback {
        callback,
        user_data,
    };

    imp.forall(&wrap, from_glib(include_internals), &callback)
}

#[derive(Debug)]
pub struct Callback {
    callback: gtk_sys::GtkCallback,
    user_data: glib_sys::gpointer,
}

impl Callback {
    pub fn call(&self, widget: &Widget) {
        unsafe {
            if let Some(callback) = self.callback {
                callback(widget.to_glib_none().0, self.user_data);
            }
        }
    }
}
