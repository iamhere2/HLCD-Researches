#![allow(unused)]

// Children modules
mod data;
mod storage_interface;
mod player_interface;
mod rules_engine;
mod file_storage;
mod console_ui;
mod game_flow;
mod ai_player;

use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;

use crate::hlcd_infra::console_app_interface::*;
use crate::hlcd_infra::console_io_interface::*;
use crate::hlcd_infra::file_io_interface::*;

use console_ui::ConsoleUI;
use file_storage::FileStorage;

use self::ai_player::AiPlayer;
use self::console_ui::ConsoleUIRef;
use self::file_storage::FileStorageRef;
use self::game_flow::component::GameFlow;
use self::game_flow::interface::*;
use self::player_interface::SyncPlayerProvider;
use self::rules_engine::component::RulesEngine;
use self::rules_engine::interface::RulesEngineProvider;
use self::storage_interface::*;

// Root component
// Provides: ConsoleApp (delegated)
// Consumes: ConsoleIO, FileIO
// Children: ConsoleUI, FileStorage, AiPlayer, GameFlow, InteractivePlayerAdapter, RulesEngine
pub struct ChessApp {
    // Owned state
    // -

    // Owned dependencies
    // -

    // Children components
    console_ui: ConsoleUIRef,
    file_storage: FileStorageRef
}

impl ChessApp {

    // Constructor with dependencies
    pub(super) fn new(
        console_io: ConsoleIORef,
        file_io: FileIORef
    ) -> ChessApp {
        // Create & connect children components
        let file_storage = Rc::new(RefCell::new(FileStorage::new(Rc::clone(&file_io))));
        let storage_interface = StorageProvider::get(Rc::clone(&file_storage));

        let ai_player = Rc::new(RefCell::new(AiPlayer::new()));
        let ai_player_interface = SyncPlayerProvider::get(Rc::clone(&ai_player));

        let rules_engine = Rc::new(RefCell::new(RulesEngine::new()));
        let rules_engine_interface = RulesEngineProvider::get(Rc::clone(&rules_engine));

        let game_flow = Rc::new(RefCell::new(GameFlow::new(
            &ai_player_interface,
            &rules_engine_interface
        )));
        let game_flow_interface = GameFlowProvider::get(Rc::clone(&game_flow));
        let flow_play_interface = FlowPlayProvider::get(Rc::clone(&game_flow));
        
        let console_ui = Rc::new(RefCell::new(ConsoleUI::new(
            &Rc::clone(&console_io), 
            &Rc::clone(&storage_interface),
            &Rc::clone(&game_flow_interface),
            &Rc::clone(&flow_play_interface),
            &Rc::clone(&rules_engine_interface)
        )));

        // Instantiate this component and assign children
        let chess_app = ChessApp { console_ui, file_storage };
        
        chess_app
    }

    // Component accessors for internal usage
    fn console_ui(&self) -> Ref<ConsoleUI> {
        self.console_ui.borrow()
    }

    fn console_ui_mut(&self) -> RefMut<ConsoleUI> {
        self.console_ui.borrow_mut()
    }
    
    // Owned dependencies
    // -
}

// Owned provided interfaces
// -

// Provided interfaces, delegated to children
impl ConsoleAppProvider for ChessApp {
    fn get(it: Rc<RefCell<Self>>) -> ConsoleAppRef {
        ConsoleAppProvider::get(Rc::clone(&it.borrow().console_ui))
    }
}
