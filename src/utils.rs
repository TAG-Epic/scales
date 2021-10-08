use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct PackageInfo {
    version: String
}
#[derive(Serialize, Deserialize, Debug)]
struct PypiInfo {
    info: PackageInfo
}
pub fn get_latest_pypi_release(package_name: &str) -> String {
    let url = &format!("https://pypi.org/pypi/{}/json", package_name).to_string();
    let body = ureq::get(url)
        .set("User-Agent", "Scales (@tag-epic on matrix)")
        .call()
        .unwrap()
        .into_string()
        .unwrap();
    let pypi_info: PypiInfo = serde_json::from_str(&body).unwrap();
    pypi_info.info.version

}
