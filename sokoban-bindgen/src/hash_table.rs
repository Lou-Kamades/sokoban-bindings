pub use sokoban::{FromSlice, HashTable, NodeAllocatorMap};

#[macro_export]
macro_rules! hash_table_bindings {
    ($key:ty, $value:ty, $buckets:literal, $size:literal) => {

        ____ci!(hash_table_name = HashTable, $key, $value, $buckets, $size {

            #[allow(non_snake_case)]
            pub mod hash_table_name {
                use super::*;

                const SIZE: usize = core::mem::size_of::<hash_table_name>();
                ____ci!(hash_table_size_name = HashTable, $key, $value, $buckets, $size, SIZE {
                    #[no_mangle]
                    pub static hash_table_size_name: usize = SIZE;
                });

                #[allow(non_upper_case)]
                pub struct hash_table_name {
                    inner: HashTable<$key, $value, $buckets, $size>
                }

                // Ensure we can use FAILURE as failure
                const _: () = assert!($size < FAILURE as usize);

                #[no_mangle]
                pub extern "C" fn initialize_table(slf: &mut hash_table_name) {
                    slf.inner.initialize()
                }

                #[no_mangle]
                pub unsafe extern "C" fn initialize_table_in(bytes: *mut u8, len: usize) -> *mut hash_table_name {
                    let byte_slice = unsafe { core::slice::from_raw_parts_mut(bytes, len)};
                    // SAFETY: transparent type
                    HashTable::<$key, $value, $buckets, $size>::new_from_slice(byte_slice) as *mut _ as *mut hash_table_name
                }

                #[no_mangle]
                pub unsafe extern "C" fn table_insert(slf: &mut hash_table_name, key: $key, value: $value) -> u32 {
                    match slf.inner.insert(key, value) {
                        Some(addr) => addr,
                        None => FAILURE,
                    }
                }

                /// Returns 0 if successful, u32::MAX if failure.
                ///
                /// If this fails, the given pointer will point to whatever was there before,
                /// which is potentially uninitialized
                #[no_mangle]
                pub unsafe extern "C" fn table_get(slf: &mut hash_table_name, key: &$key, value: *mut $value) -> u32 {
                    match slf.inner.get(key) {
                        Some(val) => {
                            unsafe { *value = *val; }
                            SUCCESS
                        },
                        None => FAILURE
                    }
                }

                /// Returns 0 if successful, u32::MAX if failure.
                ///
                /// If this fails, the given pointer will point to whatever was there before,
                /// which is potentially uninitialized
                #[no_mangle]
                pub unsafe extern "C" fn table_remove(slf: &mut hash_table_name, key: &$key, value: *mut $value) -> u32 {
                    match slf.inner.remove(key) {
                        Some(val) => {
                            unsafe {
                                *value = val;
                            }
                            SUCCESS
                        },
                        None => FAILURE,
                    }
                }
            }

        });

    };
}
