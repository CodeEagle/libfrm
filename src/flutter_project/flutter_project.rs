use glob::glob;
use std::{
  collections::HashMap,
  fs::File,
  io::{Read, Write},
  path::Path,
};
extern crate notify;
use crate::flutter_project::frm_rc::FrmRC;

use super::const_config::{FRM_CONFIG_FILENAME, FRM_INIT_CONTENT};
use notify::{EventHandler, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct FlutterProject {
  pub path: String,
  pub config: Option<FrmRC>,
  watchers: HashMap<String, RecommendedWatcher>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FlutterProjectLite {
  pub path: String,
  pub config: Option<FrmRC>,
}
// Instance
impl FlutterProject {
  /// 监听 .frmrc.json 文件
  pub fn init_frm(&mut self) {
    let frm_file_path = Path::new(&self.path).join(FRM_CONFIG_FILENAME);
    if self.config.is_none() {
      let mut file = File::create(&frm_file_path).unwrap();
      let _ = file.write(FRM_INIT_CONTENT.as_bytes()).unwrap();
      let frm: FrmRC = serde_json::from_str(&FRM_INIT_CONTENT).unwrap();
      self.config = Some(frm);
    }

    fn event_fn(res: Result<notify::Event, notify::Error>) {
      match res {
        // TODO: 监听文件更改, enable 和 assets 的配置
        Ok(event) => println!("event: {:?}", event),
        Err(e) => println!("watch error: {:?}", e),
      }
    }
    self.add_watcher(frm_file_path.to_str().unwrap().to_string(), event_fn);
  }

  fn add_watcher<F>(&mut self, path: String, event_handler: F)
  where
    F: EventHandler,
  {
    if let Ok(mut watcher) = notify::recommended_watcher(event_handler) {
      _ = watcher.watch(Path::new(&path), RecursiveMode::Recursive);
      self.watchers.insert(path, watcher);
    }
  }

  /// 切换是否监听
  pub fn toggle_watch(&mut self) -> bool {
    if self.config.is_none() {
      return false;
    }

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
          fn event_fn(res: Result<notify::Event, notify::Error>) {
            match res {
              // TODO: 监听 资源文件夹
              Ok(event) => println!("event: {:?}", event),
              Err(e) => println!("watch error: {:?}", e),
            }
          }
          self.add_watcher(watch_path, event_fn);
        }
      }
    }
    return to_watch == true;
  }
}

/// Static
impl FlutterProject {
  pub fn to_lite(target: &FlutterProject) -> FlutterProjectLite {
    FlutterProjectLite {
      path: target.path.clone(),
      config: target.config.clone(),
    }
  }
  pub fn get_all_project_of(folder: String) -> std::io::Result<Vec<FlutterProject>> {
    let mut v: Vec<FlutterProject> = vec![];
    let target_folder: String;
    if folder.ends_with("/") {
      target_folder = folder;
    } else {
      target_folder = folder + "/";
    }
    let regex = target_folder + "**/pubspec.yaml";
    for entry in glob(&regex) {
      entry.for_each(|item| {
        let path = item.unwrap();
        let raw_dir = path.parent().unwrap();
        let frm_rc_json_path = raw_dir.join(FRM_CONFIG_FILENAME);
        if let Ok(mut file) = File::open(frm_rc_json_path) {
          let mut content = String::new();
          _ = file.read_to_string(&mut content);
          let frm: FrmRC = serde_json::from_str(&content).unwrap();
          let project = FlutterProject {
            path: raw_dir.to_str().unwrap().to_string() + "/",
            config: Some(frm),
            watchers: HashMap::new(),
          };
          v.push(project.into());
        } else {
          let project = FlutterProject {
            path: raw_dir.to_str().unwrap().to_string(),
            config: None,
            watchers: HashMap::new(),
          };
          v.push(project.into());
        }
      })
    }
    return Ok(v);
  }
}

//
#[cfg(test)]
mod tests {
  // use std::env;
  use super::FlutterProject;

  #[test]
  fn list_all_pubspec() {
    // let dir = env::current_dir().unwrap().to_str().unwrap().to_string();
    let dir = "/Users/lincoln/Downloads/flutter_r_demo-master".to_string();
    if let Ok(mut list) = FlutterProject::get_all_project_of(dir) {
      let mut first = list.remove(0);
      first.init_frm();
      first.toggle_watch();
      list.insert(0, first);
      println!("{:?}, {:?}", list.len(), list);
      loop {}
    }
  }
}
