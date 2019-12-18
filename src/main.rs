use std::io::{stdin, stdout};
use std::io::Write;
use std::fs;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use std::io::Error;
use std::ffi::OsString;

const RESULT_PATH:&str = "_按页分类结果";
const FIRST_PAGE:&str = "第一页";
const OTHER_PAGE:&str = "其他页";
const FIRST_PAGE_PATTERN:&str = "第1页";

///
/// Business
///
fn main() {
    let mut path = get_path_and_check();

    println!("当前路径: {:?}\n", path);
    let files = read_dir(path.as_path(), "png");
    if files.len() == 0 {
        println!("目录下没有png格式图片");
        wait_for_end();
    }

    let (first_page, other_page) = create_result_dir(path.as_path());
    copy_images(files, first_page, other_page);

    wait_for_end();
}


///
/// Business
///
/// # 复制图片
///
fn copy_images(all_files: Vec<PathBuf>, first_path: PathBuf, other_path: PathBuf) {
    let (first_page, other_page) = bin_category(all_files, FIRST_PAGE_PATTERN);

    copy(first_page, &first_path);
    copy(other_page, &other_path);

    println!("按页码分类完成\n")
}

///
/// Business
///
/// # 创建结果文件保存目录
///
fn create_result_dir(input_path: &Path) -> (PathBuf, PathBuf) {
    let mut pathname = get_filename(input_path);
    pathname.push(RESULT_PATH);

    let root_path = match input_path.parent() {
        Some(parent) => {
            parent
        },
        None => {
            input_path
        }
    };

    let new_pathname = root_path.join(Path::new(&pathname));
    let first_page = new_pathname.join(Path::new(FIRST_PAGE));
    let other_page = new_pathname.join(Path::new(OTHER_PAGE));

    create_dir(&new_pathname);
    create_dir(&first_page);
    create_dir(&other_page);

    (first_page, other_page)
}

///
/// Util
///
/// # 获取目录并检查
///
fn get_path_and_check() -> PathBuf {
    loop {
        print!("输入目录 > ");
        stdout().flush().expect("ERROR：标准输出刷新异常！");
        if let Ok(path) = get_string_from_stdin() {
            let path = Path::new(&path);
            if path.is_dir() {
                return path.to_owned();
            } else {
                println!("输入不是目录\n");
            }
        }
    }
}

///
/// Util
///
/// # 从标准输入获取输入
///
fn get_string_from_stdin() -> Result<String, Error> {
    let mut input = String::new();
    let bytes = stdin().read_line(&mut input)?;
    if bytes > 1 {
        input = input.trim().to_owned();
    }
    Ok(input)
}

///
/// Util
///
/// # 读取目录下图片文件元信息
///
fn read_dir(path: &Path, file_type: &str) -> Vec<PathBuf> {
    let read_dir = fs::read_dir(path).expect("ERROR：读取目录失败");
    let mut files:Vec<PathBuf> = Vec::new();

    for res in read_dir {
        let entry = res.expect("ERROR：目录内容读取失败");
        let file_info = entry.path();

        if file_info.is_file() && file_info.extension() == Some(OsStr::new(file_type)) {
            files.push(file_info);
        }
    }
    files
}

///
/// Util
///
/// # 获取目录最后一级目录或文件名称
///
fn get_filename(file_path: &Path) -> OsString {
    if let Some(name) = file_path.iter().last() {
        return name.to_os_string();
    }
    OsString::new()
}

///
/// Util
///
/// # 创建目录
///
fn create_dir(new_path: &PathBuf) {
    if !new_path.exists() {
        fs::create_dir(new_path.as_path()).expect("ERROR：创建目录异常");
    }
}

///
/// Util
///
/// # 按任意键退出程序
///
fn wait_for_end() {
    print!("按任意键退出程序...");
    stdout().flush().expect("ERROR：标准输出刷新异常！");
    let _ = get_string_from_stdin();
    std::process::exit(0);
}

///
/// Util
///
/// # 复制到制定目录
///
fn copy(files: Vec<PathBuf>, target: &PathBuf) {
    for from in files {
        let name = from.file_name().expect("ERROR：复制时获取文件名称失败");
        let to = target.join(name);
        let _ = fs::copy(from, to).expect("ERROR：复制文件异常");
    }
}

///
/// Util
///
/// # 按名称关键字分为两类
///
fn bin_category(collection: Vec<PathBuf>, keyword: &str) -> (Vec<PathBuf>, Vec<PathBuf>) {

    let (mut one, mut other) = (Vec::<PathBuf>::new(), Vec::<PathBuf>::new());

    let _ = collection
        .iter()
        .map(|v| {
            if let Some(_) = v.to_string_lossy().find(keyword) {
                one.push(*v);
            } else {
                other.push(*v);
            }
        })
        .collect::<Vec<_>>();

    (one, other)
}