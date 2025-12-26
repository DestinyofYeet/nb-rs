use std::path::PathBuf;

use crate::config::model::Config;

pub struct Test {
    pub dir: PathBuf,
    pub config: Config,
}

impl Test {
    const TEST_DIR: &str = "/tmp/nb-rs_test_dir";

    pub fn setup(test_name: &str) -> Test {
        let mut path = PathBuf::new();
        path.push(Self::TEST_DIR);
        path.push(test_name);

        if path.exists() {
            Test::cleanup(&path);
        }

        println!("Creating {}", path.to_str().unwrap());

        std::fs::create_dir_all(&path).unwrap();

        std::env::set_current_dir(&path).unwrap();

        let config = Config {
            data_dir: path.clone(),
            editor: "TEST".to_string(),
            offline: true,
            is_test: true,
        };

        Test { dir: path, config }
    }

    fn cleanup(path: &PathBuf) {
        println!("Cleaning up {}", path.to_str().unwrap());

        std::fs::remove_dir_all(path).expect("Failed to clean up test directory");
    }
}

impl Drop for Test {
    fn drop(&mut self) {
        Test::cleanup(&self.dir);
    }
}
