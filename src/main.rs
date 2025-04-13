use potree_auth::{application::initialize_application, config::ApplicationConfiguration};

fn main() {
    let config = ApplicationConfiguration {
        projects_dir: "".parse().unwrap(),
    };
    let application = initialize_application(&config);
    dbg!(application);
}
