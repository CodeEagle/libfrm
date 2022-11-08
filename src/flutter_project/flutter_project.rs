use glob::glob;
use serde_yaml::Value;
use std::{collections::HashMap, fs::File, io::Read, path::Path};
extern crate notify;
use super::config_info::ConfigInfo;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct FlutterProject {
  pub path: String,
  pub name: String,
  pub config: Option<ConfigInfo>,
  watchers: HashMap<String, RecommendedWatcher>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FlutterProjectLite {
  pub path: String,
  pub name: String,
  pub config: Option<ConfigInfo>,
}
// Instance
impl FlutterProject {
  pub fn to_lite(target: &FlutterProject) -> FlutterProjectLite {
    FlutterProjectLite {
      path: target.path.clone(),
      name: target.name.clone(),
      config: target.config.clone(),
    }
  }
  pub fn toggle_watch(&mut self) -> bool {
    let to_watch = self.watchers.keys().len() == 0;
    if let Some(flr_value) = &self.config {
      let paths = flr_value.asset_root_path();

      for ele in paths {
        let mut watch_path = self.path.to_owned();
        watch_path.push_str(&ele);

        if !to_watch {
          if let Some(mut wc) = self.watchers.remove(&ele) {
            _ = wc.unwatch(Path::new(&watch_path));
          }
        } else {
          if let Ok(mut watcher) = notify::recommended_watcher(|res| match res {
            Ok(event) => println!("event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
          }) {
            _ = watcher.watch(Path::new(&watch_path), RecursiveMode::Recursive);
            println!("watch: {:?}", &watch_path);
            self.watchers.insert(watch_path, watcher);
          }
        }
      }
    }
    return to_watch == true;
  }
}
// Static
impl FlutterProject {
  pub fn get_all_project_of(folder: String) -> std::io::Result<Vec<FlutterProject>> {
    let mut v: Vec<FlutterProject> = vec![];
    let regex = folder + "/**/pubspec.yaml";
    for entry in glob(&regex) {
      entry.for_each(|item| {
        if let Ok(path) = item {
          let raw_path = path.as_path();
          if let Ok(mut file) = File::open(raw_path) {
            let mut content = String::new();
            _ = file.read_to_string(&mut content);
            if let Ok(value) = serde_yaml::from_str::<Value>(&content) {
              if let (Some(name), Some(p)) = (value["name"].as_str(), raw_path.to_str()) {
                let info = value["flr"].as_mapping();
                let paths = p.split("/");
                let mut vec1: Vec<&str> = paths.collect();
                vec1.remove(vec1.len() - 1);
                let project = FlutterProject {
                  path: vec1.join("/") + "/",
                  name: name.to_string(),
                  config: ConfigInfo::from(info),
                  watchers: HashMap::new(),
                };
                v.push(project.into());
              }
            }
          }
        }
      })
    }

    return Ok(v);
  }
}

//
#[cfg(test)]
mod tests {
  use crate::utils::get_current_working_dir;

  use super::FlutterProject;

  #[test]
  fn list_all_pubspec() {
    let dir = get_current_working_dir()
      .unwrap()
      .to_str()
      .unwrap()
      .to_string();
    print!("dir: {:?}", &dir);
    if let Ok(mut list) = FlutterProject::get_all_project_of(dir) {
      println!("{:?}, {:?}", list.len(), list);
      let mut first = list.remove(0);
      first.toggle_watch();
      list.insert(0, first);
      loop {}
    }
  }
}
