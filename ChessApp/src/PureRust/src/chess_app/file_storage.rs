use crate::hlcd_infra::file_io::FileIOInterface;

pub(super) struct FileStorage<'a> {
    
    // Owned dependencies
    file_io: Option<&'a Box<&'a dyn FileIOInterface>>
}

impl<'a> FileStorage<'a> {

    pub(super) fn new() -> FileStorage<'a> {
        FileStorage { file_io: None }
    }

    // Owned dependencies
    pub(super) fn set_file_io(&mut self, file_io: &'a Box<&'a dyn FileIOInterface>) {
        self.file_io = Some(file_io);
    }

    fn get_file_io(&self) -> &Box<&dyn FileIOInterface> {
        self.file_io.as_ref().unwrap()
    }
}