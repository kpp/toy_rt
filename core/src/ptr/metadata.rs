
#[lang = "pointee_trait"]
pub trait Pointee {
    /// The type for metadata in pointers and references to `Self`.
    #[lang = "metadata_type"]
    // NOTE: Keep trait bounds in `static_assert_expected_bounds_for_metadata`
    // in `library/core/src/ptr/metadata.rs`
    // in sync with those here:
    type Metadata: Copy; // + Send + Sync + Ord + Hash + Unpin;
}

pub fn metadata<T: ?Sized>(ptr: *const T) -> <T as Pointee>::Metadata {
    // SAFETY: Accessing the value from the `PtrRepr` union is safe since *const T
    // and PtrComponents<T> have the same memory layouts. Only std can make this
    // guarantee.
    unsafe { PtrRepr { const_ptr: ptr }.components.metadata }
}

#[repr(C)]
pub(crate) union PtrRepr<T: ?Sized> {
    pub(crate) const_ptr: *const T,
    pub(crate) mut_ptr: *mut T,
    pub(crate) components: PtrComponents<T>,
}

#[repr(C)]
pub(crate) struct PtrComponents<T: ?Sized> {
    pub(crate) data_address: *const (),
    pub(crate) metadata: <T as Pointee>::Metadata,
}

// Manual impl needed to avoid `T: Copy` bound.
impl<T: ?Sized> Copy for PtrComponents<T> {}

// Manual impl needed to avoid `T: Clone` bound.
impl<T: ?Sized> Clone for PtrComponents<T> {
    fn clone(&self) -> Self {
        *self
    }
}

#[inline]
pub const fn from_raw_parts<T: ?Sized>(
    data_address: *const (),
    metadata: <T as Pointee>::Metadata,
) -> *const T {
    // SAFETY: Accessing the value from the `PtrRepr` union is safe since *const T
    // and PtrComponents<T> have the same memory layouts. Only std can make this
    // guarantee.
    unsafe { PtrRepr { components: PtrComponents { data_address, metadata } }.const_ptr }
}

#[inline]
pub const fn from_raw_parts_mut<T: ?Sized>(
    data_address: *mut (),
    metadata: <T as Pointee>::Metadata,
) -> *mut T {
    // SAFETY: Accessing the value from the `PtrRepr` union is safe since *const T
    // and PtrComponents<T> have the same memory layouts. Only std can make this
    // guarantee.
    unsafe { PtrRepr { components: PtrComponents { data_address, metadata } }.mut_ptr }
}
