use crate::config;
use crate::utils;
use std::fs::create_dir;
use std::thread;
use ureq;
use std::fs::File;
use std::io::BufWriter;
use std::io::Read;

pub fn run() {
    let config = config::get_config();
    let state_modules_path = utils::modules_path();
    
    let mut threads: Vec<thread::JoinHandle<()>> = Vec::new();

    for dependency in config.dependencies {
        match dependency {
            config::Dependency::Pypi(dep) => {
                let dep_path = state_modules_path.join(format!("pypi-{}-{}", dep.name, dep.version));
                if !dep_path.exists() {
                    let thread_dep = dep.clone();
                    let thread = thread::spawn(move || {install_pypi(&thread_dep);});
                    threads.push(thread);
                }
            },
            config::Dependency::Git(_) => utils::error("Git dependencies can't be installed yet.")
        };
    }
    for thread in threads {
        thread.join().expect("Crashed thread?");
    }
}

fn install_pypi(dependency: &config::PypiDependency) {
    let state_modules = utils::modules_path();
    let module_path = state_modules.join(format!("pypi-{}-{}/", dependency.name, dependency.version));

    if module_path.exists() {
        panic!("Pypi module already exists for package {}", dependency.name);
    }

    let project_info = utils::get_pypi_info(&dependency.name);
    let download_url = project_info
        .get("releases").unwrap()
        .get(&dependency.version).unwrap()
        .get(0).unwrap()
        .get("url").unwrap().as_str().unwrap();

    println!("Download url: {}", download_url);
    
    {
        let file_path = format!("/tmp/scales-pypi-{}-{}", dependency.name, dependency.version).to_string();
        let file = match File::create(&file_path) {
            Err(error) => panic!("Couldn't open config file: {}", error),
            Ok(file) => file
        };
        let writer = BufWriter::new(file);

        let r = ureq::get(download_url)
            .call().unwrap()
            .into_reader().read_to_end(&mut writer);

    } 

    // Put successfully installed files in the shared package dir
    create_dir(module_path).unwrap();
}

