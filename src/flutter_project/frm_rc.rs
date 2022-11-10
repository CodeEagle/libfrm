use std::{cmp::min, collections::HashSet};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FrmRC {
  pub assets: Vec<String>,
  pub enable: bool,
}

/// Instance
impl FrmRC {
  fn gcd_of_string(str1: &String, str2: &String) -> String {
    let str1_paths = str1.split("/");
    let str2_paths = str2.split("/");
    let vec1: Vec<&str> = str1_paths.collect();
    let vec2: Vec<&str> = str2_paths.collect();
    let len1 = vec1.len();
    let len2 = vec2.len();
    let len = min(&len1, &len2);
    let mut index: usize = 0;
    fn build_result(index: usize, target: Vec<&str>) -> String {
      let mut real_index = 0;
      let mut list_to: Vec<&str> = vec![];
      while real_index < index {
        //
        list_to.push(target[real_index]);
        real_index += 1;
      }
      let str = list_to.join("/") + "/";
      return str;
    }
    while index < len - 1 {
      let index1_value = vec1[index];
      let index2_value = vec2[index];
      if !index1_value.eq(index2_value) {
        if index == 0 {
          return "".to_string();
        } else {
          return build_result(index, vec1);
        }
      }
      index += 1;
      if index >= len - 1 {
        return build_result(index, vec1);
      }
    }
    return "".to_string();
  }
  /**
   * 找出 assets 的公共目录, 监听
   */
  pub fn asset_root_path(&self) -> HashSet<String> {
    let mut set: HashSet<String> = HashSet::new();
    if self.assets.is_empty() {
      return set;
    }
    if self.assets.len() == 1 {
      let path = self.assets.first().unwrap();
      set.insert(path.clone());
      return set;
    } else {
      let mut a2 = self.assets.to_vec();
      a2.reverse();

      for ele in &self.assets {
        for rele in &a2 {
          let gcd = Self::gcd_of_string(ele, rele);
          if !gcd.is_empty() {
            set.insert(gcd);
          }
        }
      }
      return set;
    }
  }
}
