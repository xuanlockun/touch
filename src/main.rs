use std::{env, fs, io};

fn main() -> io::Result<()> {
    let filename = match env::args().nth(1) {
        Some(name) => name,
        None => {
            println!("Vui lòng nhập tên file");
            return Ok(());
        }
    };
    match fs::File::create(&filename) {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("Lỗi khi tạo file {} {}",filename, e);
            return Ok(());
        }
    }
}
