use std::io::{stdin, stdout};
use std::io::Write;
use std::fs;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

const RESULT_PATH:&str = "结果";

fn main() {
    let mut path = String::new();
    print!("{}", path);

    loop {
        print!("目录 > ");
        stdout().flush().unwrap();
        path = get_path_string_from_input();

        if is_dir(&path) {
            break;
        }
    }

    println!("当前路径: {}\n", path);
    let files = read_dir(&path);
    if files.len() == 0 {
        panic!("目录为空");
    }

    let result_path = create_result_root(&path);
    page(&result_path, &files);

    print!("请按任意键退出程序...");
    stdout().flush().unwrap();
    let _ = get_path_string_from_input();
}

fn get_path_string_from_input() -> String {
    let mut path = String::new();
    let ret = stdin().read_line(&mut path).unwrap();
    if ret > 1 {
        path = path.trim().to_owned();
    }
    path
}

fn is_dir(path: &str) -> bool {
    let tmp_path = Path::new(path);
    tmp_path.is_dir()
}

fn read_dir(path: &str) -> Vec<PathBuf> {
    let read_dir = fs::read_dir(path).unwrap();
    let mut files:Vec<PathBuf> = Vec::new();

    for res in read_dir {
        let entry = res.unwrap();
        let path_buf = entry.path();

        if path_buf.is_file() && path_buf.extension() == Some(OsStr::new("png")) {
            files.push(path_buf);
        }
    }
    files
}

fn create_result_root(base_path: &str) -> PathBuf {
    let path = Path::new(base_path);
    let new_root = path.parent().unwrap().join(Path::new(RESULT_PATH));
    if !new_root.exists() {
        fs::create_dir(new_root.as_path()).unwrap();
    }
    new_root
}

fn page(result_path: &PathBuf, all: &Vec<PathBuf>) {
    let first_page_path = result_path.join(Path::new("第一页"));
    let other_page_path = result_path.join(Path::new("其他页"));

    if !first_page_path.exists() {
        fs::create_dir(first_page_path.as_path()).unwrap();
        if first_page_path.exists() {
            println!("第一页 目录创建成功...");
        }
    }

    if !other_page_path.exists() {
        fs::create_dir(other_page_path.as_path()).unwrap();
        if other_page_path.exists() {
            println!("其他页 目录创建成功...");
        }
    }

    let first = all.iter().filter(|f| {
        let s = f.to_str().unwrap();
        if let Some(_) = s.find("第1页") {
            return true;
        }
        false
    }).collect::<Vec<_>>();

    let other = all.iter().filter(|f| {
        let s = f.to_str().unwrap();
        if let None = s.find("第1页") {
            return true;
        }
        false
    }).collect::<Vec<_>>();


    for from in first {
        let name = from.file_name().unwrap();
        let to = first_page_path.join(name);
        let _ = fs::copy(from, to).unwrap();
    }

    for from in other {
        let name = from.file_name().unwrap();
        let to = other_page_path.join(name);
        let _ = fs::copy(from, to).unwrap();
    }
    println!("按页码分类完成")
}