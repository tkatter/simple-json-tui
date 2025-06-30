use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use crate::app::CurrentScreen;

#[derive(Default)]
pub struct FileState {
    pub next_screen: CurrentScreen,
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

    pub fn remove_file(&mut self) {
        let file = self.file.take();
        file.expect("Function is only called if a file exists")
            .flush()
            .unwrap();
        std::fs::remove_file(&self.fname_input).unwrap();
    }
}
