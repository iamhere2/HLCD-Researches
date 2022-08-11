mod console_ui;
mod file_storage;

use crate::hlcd_infra::{console_app::*, console_io::ConsoleIOInterface, file_io::FileIOInterface};
use console_ui::ConsoleUI;
use file_storage::FileStorage;

pub struct ChessApp<'a> {
    // Owned dependencies
    // -

    // Children components
    console_ui: ConsoleUI<'a>,
    file_storage: FileStorage<'a>
}

impl<'a> ChessApp<'a> {

    pub(super) fn new() -> ChessApp<'a> {
        // Create children components
        let console_ui = ConsoleUI::new();
        let file_storage = FileStorage::new();

        // Instantiate this component and assign children
        let chess_app = ChessApp { console_ui, file_storage };

        // Link children components to each other
        // -

        chess_app
    }

    // Delegated dependencies
    pub(super) fn set_console_io(&mut self, console_io: &'a Box<&'a dyn ConsoleIOInterface>) {
        self.console_ui.set_console_io(console_io);
    }

    pub(super) fn set_file_io(&mut self, file_io: &'a Box<&'a dyn FileIOInterface>) {
        self.file_storage.set_file_io(file_io);
    }

    // Delegated provided interfaces
    pub(super) fn get_console_app(&self) -> Box<&dyn ConsoleAppInterface> {
        self.console_ui.get_console_app()
    }

    // Owned provided interfaces
    // -

    // Owned dependencies
    // -
}
