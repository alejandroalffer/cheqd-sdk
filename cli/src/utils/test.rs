use crate::utils::environment::EnvironmentUtils;

use std::fs;

pub struct TestUtils {}

impl TestUtils {
    pub fn cleanup_indy_home() {
        let path = EnvironmentUtils::indy_home_path();
        if path.exists() {
            fs::remove_dir_all(path).unwrap();
        }
    }

    pub fn cleanup_cheqd_home() {
        let path = EnvironmentUtils::cheqd_home_path();
        if path.exists() {
            fs::remove_dir_all(path).unwrap();
        }
    }

    pub fn cleanup_temp() {
        let path = EnvironmentUtils::tmp_path();
        if path.exists() {
            fs::remove_dir_all(path).unwrap();
        }
    }

    pub fn cleanup_storage() {
        TestUtils::cleanup_indy_home();
        TestUtils::cleanup_cheqd_home();
        TestUtils::cleanup_temp();
    }
}
