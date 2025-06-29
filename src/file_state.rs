use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

#[derive(Default)]
pub struct FileState {
    pub fname_input: String,
    file: Option<BufWriter<File>>,
}

impl FileState {
    pub fn create_file_buf(&mut self) {
        let file_name = Path::new(&self.fname_input);

        let file = std::fs::File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_name);

        let buf: BufWriter<File> = BufWriter::new(file.unwrap());

        self.file = Some(buf);
    }

    pub fn write_file(&mut self, json: String) -> Result<(), Box<dyn std::error::Error>> {
        let json_bytes = json.as_bytes();

        match &mut self.file {
            Some(file) => {
                file.write_all(json_bytes)?;
                file.flush()?;
                Ok(())
            }
            None => Ok(()),
        }
    }
}

// #[allow(unused_variables, unused_mut)]
// let mut tmp_file: BufWriter<File> = create_tmp_file().unwrap();
// tmp_file.write_all(b"tedt").unwrap();
// tmp_file.flush().unwrap();
// let mut tmp_file = match create_tmp_file() {
//     Some(file) => file,
//     None => {
//         println!("Failed to create temporary file for JSON storage");
//         restore_terminal(terminal)?;
//     }
// };
