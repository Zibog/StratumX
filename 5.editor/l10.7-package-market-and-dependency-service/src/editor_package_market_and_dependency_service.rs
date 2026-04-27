pub use serde::{Deserialize, Serialize};
pub use serde_json;
pub use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    sync::Arc,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageMarketAndDependencyService {
    pub installed_packages: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorProduct {
    pub package_service: PackageMarketAndDependencyService,
}

impl EditorProduct {
    pub fn add_package(&mut self, package: impl Into<String>) {
        let package = package.into();
        if !self.package_service.installed_packages.contains(&package) {
            self.package_service.installed_packages.push(package);
        }
    }
}
