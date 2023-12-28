#[repr(transparent)]
pub struct SharedPtr<T> {
    pub ptr: *mut T,
}
unsafe impl<T: Send> Send for SharedPtr<T> {}
unsafe impl<T: Sync> Sync for SharedPtr<T> {}

impl<T> SharedPtr<T> {
    pub fn new(ptr: *mut T) -> Self {
        SharedPtr { ptr }
    }

    delegate::delegate! {
        to self.ptr {
            /// # Safety
            ///
            /// Ensure that `count` does not exceed the capacity of the Vec.
            pub unsafe fn add(&self, count: usize) -> *mut T;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;

    #[test]
    fn update_on_vec_in_multi_threads() {
        let mut avec = vec![0; 3];
        let shptr = SharedPtr::new(avec.as_mut_ptr());
        thread::scope(|s| {
            s.spawn(|| {
                unsafe {
                    *shptr.add(1) = 1;
                }
            });
            s.spawn(|| {
                unsafe {
                    *shptr.add(2) = 2;
                }
            });
        });
        assert!(avec == vec![0, 1, 2]);
    }
}