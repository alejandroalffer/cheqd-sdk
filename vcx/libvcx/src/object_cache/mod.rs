use rand::Rng;

use error::prelude::*;
use dashmap::DashMap;

pub struct ObjectCache<T> {
    store: DashMap<u32, T>
}

impl<T> Default for ObjectCache<T> {
    fn default() -> ObjectCache<T>
    {
        ObjectCache {
            store: Default::default(),
        }
    }
}

impl<T> ObjectCache<T> {
    pub fn has_handle(&self, handle: u32) -> bool {
        self.store.contains_key(&handle)
    }

    pub fn get<F, R>(&self, handle: u32, closure: F) -> VcxResult<R>
        where F: Fn(&T) -> VcxResult<R> {
        closure(
            self.store
            .get(&handle)
            .ok_or(
                VcxError::from_msg(
                    VcxErrorKind::InvalidHandle,
                    format!("Object not found for handle: {}", handle)
                )
            )?
            .value()
        )
    }

    pub fn get_mut<F, R>(&self, handle: u32, closure: F) -> VcxResult<R>
        where F: Fn(&mut T) -> VcxResult<R> {
        closure(
            self.store
            .get_mut(&handle)
            .ok_or(
                VcxError::from_msg(
                    VcxErrorKind::InvalidHandle,
                    format!("Object not found for handle: {}", handle)
                )
            )?
            .value_mut()
        )
    }

    pub fn add(&self, obj: T) -> VcxResult<u32> {
        let mut new_handle = rand::thread_rng().gen::<u32>();

        loop {
            if !self.store.contains_key(&new_handle) {
                break;
            }
            new_handle = rand::thread_rng().gen::<u32>();
        }

        match self.store.insert(new_handle, obj) {
            Some(_) => Ok(new_handle),
            None => Ok(new_handle)
        }
    }

    pub fn insert(&self, handle: u32, obj: T) -> VcxResult<()> {
        match self.store.insert(handle, obj) {
            _ => Ok(()),
        }

    }

    pub fn update(&self, handle: u32, obj: T) -> VcxResult<()> {
        self.store.insert(handle, obj);
        Ok(())
    }

    pub fn release(&self, handle: u32) -> VcxResult<()> {
        match self.store.remove(&handle) {
            Some(_) => Ok(()),
            None => Err(VcxError::from_msg(VcxErrorKind::InvalidHandle, format!("Object not found for handle: {}", handle)))
        }

    }

    pub fn drain(&self) -> VcxResult<()> {
        Ok(self.store.clear())
    }
}

#[cfg(test)]
mod tests {
    use object_cache::ObjectCache;
    use utils::devsetup::SetupDefaults;
    use std::thread;

    lazy_static! {
        static ref TEST_CACHE: ObjectCache<String> = Default::default();
    }

    #[test]
    fn create_test() {
        let _setup = SetupDefaults::init();

        let _c: ObjectCache<u32> = Default::default();
    }

    #[test]
    fn get_closure() {
        let _setup = SetupDefaults::init();

        let test: ObjectCache<u32> = Default::default();
        let handle = test.add(2222).unwrap();
        let rtn = test.get(handle, |obj| Ok(obj.clone()));
        assert_eq!(2222, rtn.unwrap())
    }

    #[test]
    fn to_string_test() {
        let _setup = SetupDefaults::init();

        let test: ObjectCache<u32> = Default::default();
        let handle = test.add(2222).unwrap();
        let string: String = test.get(handle, |_| {
            Ok(String::from("TEST"))
        }).unwrap();

        assert_eq!("TEST", string);
    }

    #[test]
    fn mut_object_test() {
        let _setup = SetupDefaults::init();

        let test: ObjectCache<String> = Default::default();
        let handle = test.add(String::from("TEST")).unwrap();

        test.get_mut(handle, |obj| {
            obj.to_lowercase();
            Ok(())
        }).unwrap();

        let string: String = test.get(handle, |obj| {
            Ok(obj.clone())
        }).unwrap();

        assert_eq!("TEST", string);
    }

    #[test]
    fn multi_thread_get() {
        for i in 0 .. 2000 {
            let test_str = format!("TEST_MULTI_{}", i.to_string());
            let test_str1 = test_str.clone();
            let handle = TEST_CACHE.add(test_str).unwrap();
            let t1 = thread::spawn(move || {
                TEST_CACHE.get_mut(handle, |s| {
                    s.insert_str(0, "THREAD1_");
                    Ok(())
                }).unwrap()
            });
            let t2 = thread::spawn(move || {
                TEST_CACHE.get_mut(handle, |s| {
                    s.push_str("_THREAD2");
                    Ok(())
                }).unwrap()
            });
            t1.join().unwrap();
            t2.join().unwrap();
            TEST_CACHE.get(handle, |s| {
                let expected_str = format!("THREAD1_{}_THREAD2", test_str1);
                assert_eq!(&expected_str, s);
                Ok(())
            }).unwrap();
        }
    }
}
