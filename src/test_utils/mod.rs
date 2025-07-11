//! Helpers for automated testing.

// Constants representing the path to various dummy project directories.
pub const TEST_PROJECT_PARENT: &str = "src/test_utils/project_dir";
pub const TEST_PROJECT_1_DIR: &str = "project_1";
pub const TEST_PROJECT_1_DATA_PATH: &str = "data/some_data.txt";
pub const TEST_PROJECT_2_DIR: &str = "project_2";
pub const TEST_PROJECT_2_DATA_PATH: &str = "data/other_data.csv";

// The content of the project data
pub static TEST_PROJECT_1_DATA_CONTENT: &str =
    include_str!("project_dir/project_1/data/some_data.txt");
pub static TEST_PROJECT_1_DATA_TYPE: mime::Mime = mime::TEXT_PLAIN;
