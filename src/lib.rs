#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

pub mod flutter_project;
use flutter_project::flutter_project::FlutterProject;
use std::sync::{Arc, Mutex};

struct FLRManager {
  projects: Vec<FlutterProject>,
}

impl FLRManager {
  pub fn shared() -> Arc<Mutex<FLRManager>> {
    static mut POINT: Option<Arc<Mutex<FLRManager>>> = None;

    unsafe {
      // Rust中使用可变静态变量都是unsafe的
      POINT
        .get_or_insert_with(|| {
          // 初始化单例对象的代码
          Arc::new(Mutex::new(FLRManager { projects: vec![] }))
        })
        .clone()
    }
  }
}

#[napi]
pub fn projects_of(folder: String) -> Vec<String> {
  let result = FlutterProject::get_all_project_of(folder);
  if let Ok(projects) = result {
    let ret = projects
      .iter()
      .map(|x| {
        let a = FlutterProject::to_lite(x);
        serde_json::to_string(&a).unwrap()
      })
      .collect();
    FLRManager::shared().lock().unwrap().projects = projects;
    return ret;
  }
  return [].to_vec();
}

#[napi]
pub fn toggle_watch_for(path: String) -> bool {
  let index = FLRManager::shared()
    .lock()
    .unwrap()
    .projects
    .iter()
    .position(|x| *x.path == path)
    .unwrap();
  let mut item = FLRManager::shared().lock().unwrap().projects.remove(index);
  let value = item.toggle_watch();
  FLRManager::shared().lock().unwrap().projects.push(item);
  return value;
}
