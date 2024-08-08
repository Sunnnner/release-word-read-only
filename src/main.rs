use std::fs;
use std::io;
use std::path::Path;
use std::os::windows::fs::MetadataExt;

fn main() -> io::Result<()> {
    let current_dir = std::env::current_dir()?;
    process_directory(&current_dir)?;
    println!("处理完成。");
    Ok(())
}

fn process_directory(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                process_directory(&path)?;
            } else {
                let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
                if extension == "doc" || extension == "docx" {
                    remove_readonly(&path)?;
                }
            }
        }
    }
    Ok(())
}

fn remove_readonly(file_path: &Path) -> io::Result<()> {
    let metadata = fs::metadata(file_path)?;
    let attributes = metadata.file_attributes();
    
    // 检查是否为只读文件
    if attributes & 0x1 != 0 {
        println!("移除文件的只读属性: {:?}", file_path);
        let mut permissions = metadata.permissions();
        permissions.set_readonly(false);
        fs::set_permissions(file_path, permissions)?;
    }
    
    Ok(())
}