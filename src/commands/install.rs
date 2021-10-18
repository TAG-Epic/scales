use crate::config;
use crate::utils;
use futures::future::join_all;
use reqwest;
use tokio::io::AsyncWriteExt;
use tar::Archive;
use flate2::read::GzDecoder;
use tokio::task;
use std::fs;

pub async fn run() {
    let config = config::get_config();
    let state_modules_path = utils::modules_path();
    
    let mut tasks = Vec::new();

    for dependency in config.dependencies {
        match dependency {
            config::Dependency::Pypi(dep) => {
                let dep_path = state_modules_path.join(format!("pypi-{}-{}", dep.name, dep.version));
                if !dep_path.exists() {
                    let task_dep = dep.clone();
                    let task = install_pypi(task_dep);
                    tasks.push(task);
                }
            },
            config::Dependency::Git(_) => utils::error("Git dependencies can't be installed yet.")
        };
    }
    join_all(tasks).await;
}

async fn install_pypi(dependency: config::PypiDependency) {
    let state_modules = utils::modules_path();
    let module_path = state_modules.join(format!("pypi-{}-{}/", dependency.name, dependency.version));

    if module_path.exists() {
        panic!("Pypi module already exists for package {}", dependency.name);
    }

    let project_info = utils::get_pypi_info(&dependency.name);
    let release = project_info
        .get("releases").unwrap()
        .get(&dependency.version).unwrap()
        .get(0).unwrap();
    let download_url = release.get("url").unwrap().as_str().unwrap();
    let download_type = release.get("packagetype").unwrap().as_str().unwrap();

    if download_type != "source" && download_type != "sdist" {
        utils::error(&format!("Release type {} not supported yet", download_type).to_string());
    }
    
    let tar_path = format!("/tmp/scales-pypi-{}-{}.tar.gz", dependency.name, dependency.version);

    {
        let mut file = tokio::fs::File::create(&tar_path).await.unwrap();
        let r = reqwest::get(download_url).await.unwrap();
        file.write_all(&r.bytes().await.unwrap()).await.expect("Could not write to file.");
    }
    
    let zipped_path = format!("/tmp/scales-pypi-{}-{}.tar.gz", dependency.name, dependency.version);
    let output_path = format!("/tmp/{}-{}.tar.gz", dependency.name, dependency.version);

    let task_zipped_path = zipped_path.clone();

    task::spawn_blocking(move || {
        let file = fs::File::open(&task_zipped_path).unwrap();
        let tar = GzDecoder::new(file);
        let mut archive = Archive::new(tar);
        archive.unpack("/tmp").expect("Failed to unarchive dependency.");
    }).await.unwrap(); 
    
    install_package(&output_path).await;
}

async fn install_package(output_path: &str) {
    println!("Installed package");
}

