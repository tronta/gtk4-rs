// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{TreeDragDest, TreePath};
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{Cast, Value};

pub trait TreeDragDestImpl: ObjectImpl {
    fn drag_data_received(
        &self,
        tree_drag_dest: &Self::Type,
        dest: &TreePath,
        value: Value,
    ) -> bool;
    fn row_drop_possible(&self, tree_drag_dest: &Self::Type, dest: &TreePath, value: Value)
        -> bool;
}

pub trait TreeDragDestImplExt: ObjectSubclass {
    fn parent_drag_data_received(
        &self,
        tree_drag_dest: &Self::Type,
        dest: &TreePath,
        value: Value,
    ) -> bool;
    fn parent_row_drop_possible(
        &self,
        tree_drag_dest: &Self::Type,
        dest: &TreePath,
        value: Value,
    ) -> bool;
}

impl<T: TreeDragDestImpl> TreeDragDestImplExt for T {
    fn parent_drag_data_received(
        &self,
        tree_drag_dest: &Self::Type,
        dest: &TreePath,
        value: Value,
    ) -> bool {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().get_parent_interface::<TreeDragDest>()
                as *const ffi::GtkTreeDragDestIface;

            let func = (*parent_iface)
                .drag_data_received
                .expect("no parent \"drag_data_received\" implementation");

            from_glib(func(
                tree_drag_dest
                    .unsafe_cast_ref::<TreeDragDest>()
                    .to_glib_none()
                    .0,
                mut_override(dest.to_glib_none().0),
                value.to_glib_none().0,
            ))
        }
    }

    fn parent_row_drop_possible(
        &self,
        tree_drag_dest: &Self::Type,
        dest: &TreePath,
        value: Value,
    ) -> bool {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().get_parent_interface::<TreeDragDest>()
                as *const ffi::GtkTreeDragDestIface;

            let func = (*parent_iface)
                .drag_data_received
                .expect("no parent \"drag_data_received\" implementation");

            from_glib(func(
                tree_drag_dest
                    .unsafe_cast_ref::<TreeDragDest>()
                    .to_glib_none()
                    .0,
                mut_override(dest.to_glib_none().0),
                value.to_glib_none().0,
            ))
        }
    }
}

unsafe impl<T: TreeDragDestImpl> IsImplementable<T> for TreeDragDest {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.drag_data_received = Some(tree_drag_dest_drag_data_received::<T>);
        iface.row_drop_possible = Some(tree_drag_dest_row_drop_possible::<T>);
    }

    fn instance_init(_instance: &mut glib::subclass::InitializingObject<T>) {}
}

unsafe extern "C" fn tree_drag_dest_drag_data_received<T: TreeDragDestImpl>(
    tree_drag_dest: *mut ffi::GtkTreeDragDest,
    destptr: *mut ffi::GtkTreePath,
    valueptr: *const glib::gobject_ffi::GValue,
) -> glib::ffi::gboolean {
    let instance = &*(tree_drag_dest as *mut T::Instance);
    let imp = instance.get_impl();

    let dest: Borrowed<TreePath> = from_glib_borrow(destptr);
    let value: Value = from_glib_none(valueptr);

    imp.drag_data_received(
        from_glib_borrow::<_, TreeDragDest>(tree_drag_dest).unsafe_cast_ref(),
        &dest,
        value,
    )
    .to_glib()
}

unsafe extern "C" fn tree_drag_dest_row_drop_possible<T: TreeDragDestImpl>(
    tree_drag_dest: *mut ffi::GtkTreeDragDest,
    destptr: *mut ffi::GtkTreePath,
    valueptr: *const glib::gobject_ffi::GValue,
) -> glib::ffi::gboolean {
    let instance = &*(tree_drag_dest as *mut T::Instance);
    let imp = instance.get_impl();
    let dest: Borrowed<TreePath> = from_glib_borrow(destptr);
    let value: Value = from_glib_none(valueptr);

    imp.row_drop_possible(
        from_glib_borrow::<_, TreeDragDest>(tree_drag_dest).unsafe_cast_ref(),
        &dest,
        value,
    )
    .to_glib()
}