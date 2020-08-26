use crate::storage::list_dir;
use rocket::get;
use rocket_contrib::json::Json;
use serde::Serialize;
use std::path::PathBuf;

#[get("/api?<begin>")]
pub async fn api_index(begin: Option<String>) -> Json<List> {
    api(PathBuf::new(), begin).await
}

#[get("/api/<path..>?<begin>")]
pub async fn api(path: PathBuf, begin: Option<String>) -> Json<List> {
    let parent_path = path.parent().map(|p| p.to_string_lossy());
    let path = path.to_string_lossy();
    let result = list_dir(path.as_ref(), begin).await;

    let mut list = List {
        path: path.as_ref().to_owned(),
        parent_path: parent_path.map(|s| s.into_owned()),
        files: Vec::new(),
        directories: Vec::new(),
        next_key: None,
    };

    if let Some(result) = result {
        list.next_key = result.next_marker;

        let name_offset = if path.len() == 0 { 0 } else { path.len() + 1 };

        if let Some(files) = result.contents {
            for file in files {
                list.files.push(File {
                    name: file.key.as_ref().map_or("", |s| &s)[name_offset..].into(),
                    full_name: file.key.unwrap_or(String::new()),
                    size: file.size.unwrap_or(0),
                    e_tag: file.e_tag.map(|s| s.trim_matches('"').into()),
                    last_modified: file.last_modified,
                });
            }
        }

        if let Some(directories) = result.common_prefixes {
            for directory in directories {
                let full_name = directory.prefix.as_ref().map_or("", |s| &s[..s.len() - 1]);
                let name = &full_name[name_offset..];

                list.directories.push(Directory {
                    full_name: full_name.into(),
                    name: name.into(),
                });
            }
        }
    }

    Json(list)
}

#[derive(Serialize)]
pub struct List {
    pub path: String,
    pub parent_path: Option<String>,
    pub files: Vec<File>,
    pub directories: Vec<Directory>,
    pub next_key: Option<String>,
}

#[derive(Serialize)]
pub struct File {
    pub name: String,
    pub full_name: String,
    pub size: i64,
    pub e_tag: Option<String>,
    pub last_modified: Option<String>,
}

#[derive(Serialize)]
pub struct Directory {
    pub name: String,
    pub full_name: String,
}
