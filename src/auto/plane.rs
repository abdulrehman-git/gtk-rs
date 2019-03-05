// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Point3D;
use Vec3;
use Vec4;
use ffi;
use glib::translate::*;
use gobject_ffi;

glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct Plane(Boxed<ffi::graphene_plane_t>);

    match fn {
        copy => |ptr| gobject_ffi::g_boxed_copy(ffi::graphene_plane_get_type(), ptr as *mut _) as *mut ffi::graphene_plane_t,
        free => |ptr| gobject_ffi::g_boxed_free(ffi::graphene_plane_get_type(), ptr as *mut _),
        get_type => || ffi::graphene_plane_get_type(),
    }
}

impl Plane {
    pub fn distance(&self, point: &Point3D) -> f32 {
        unsafe {
            ffi::graphene_plane_distance(self.to_glib_none().0, point.to_glib_none().0)
        }
    }

    fn equal(&self, b: &Plane) -> bool {
        unsafe {
            from_glib(ffi::graphene_plane_equal(self.to_glib_none().0, b.to_glib_none().0))
        }
    }

    pub fn get_constant(&self) -> f32 {
        unsafe {
            ffi::graphene_plane_get_constant(self.to_glib_none().0)
        }
    }

    pub fn get_normal(&self) -> Vec3 {
        unsafe {
            let mut normal = Vec3::uninitialized();
            ffi::graphene_plane_get_normal(self.to_glib_none().0, normal.to_glib_none_mut().0);
            normal
        }
    }

    pub fn init(&mut self, normal: Option<&Vec3>, constant: f32) -> Option<Plane> {
        unsafe {
            from_glib_none(ffi::graphene_plane_init(self.to_glib_none_mut().0, normal.to_glib_none().0, constant))
        }
    }

    pub fn init_from_plane(&mut self, src: &Plane) -> Option<Plane> {
        unsafe {
            from_glib_none(ffi::graphene_plane_init_from_plane(self.to_glib_none_mut().0, src.to_glib_none().0))
        }
    }

    pub fn init_from_point(&mut self, normal: &Vec3, point: &Point3D) -> Option<Plane> {
        unsafe {
            from_glib_none(ffi::graphene_plane_init_from_point(self.to_glib_none_mut().0, normal.to_glib_none().0, point.to_glib_none().0))
        }
    }

    pub fn init_from_points(&mut self, a: &Point3D, b: &Point3D, c: &Point3D) -> Option<Plane> {
        unsafe {
            from_glib_none(ffi::graphene_plane_init_from_points(self.to_glib_none_mut().0, a.to_glib_none().0, b.to_glib_none().0, c.to_glib_none().0))
        }
    }

    pub fn init_from_vec4(&mut self, src: &Vec4) -> Option<Plane> {
        unsafe {
            from_glib_none(ffi::graphene_plane_init_from_vec4(self.to_glib_none_mut().0, src.to_glib_none().0))
        }
    }

    pub fn negate(&self) -> Plane {
        unsafe {
            let mut res = Plane::uninitialized();
            ffi::graphene_plane_negate(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn normalize(&self) -> Plane {
        unsafe {
            let mut res = Plane::uninitialized();
            ffi::graphene_plane_normalize(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }
}

impl PartialEq for Plane {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Plane {}
