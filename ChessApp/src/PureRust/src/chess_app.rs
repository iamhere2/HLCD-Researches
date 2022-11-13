#![allow(unused)]

// Children modules
mod data;
mod storage_interface;
mod player_interface;
mod rules_engine;
mod file_storage;
mod console_ui;
mod game_flow;
mod async_ai_player;
mod interactive_player_adapter;

use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

use crate::hlcd_infra::console_app_interface::*;
use crate::hlcd_infra::console_io_interface::*;
use crate::hlcd_infra::file_io_interface::*;

use console_ui::ConsoleUI;
use file_storage::FileStorage;

use self::async_ai_player::AsyncAiPlayer;
use self::game_flow::component::GameFlow;
use self::game_flow::interface::GameFlowAsyncProvider;
use self::interactive_player_adapter::{component::*, interface::*};
use self::interactive_player_adapter::interface::InteractivePlayerAdapterAsyncProvider;
use self::player_interface::AsyncPlayerProvider;
use self::rules_engine::component::RulesEngine;
use self::rules_engine::interface::RulesEngineAsyncProvider;
use self::storage_interface::StorageProvider;

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
    console_ui: Rc<RefCell<ConsoleUI>>,
    file_storage: Rc<RefCell<FileStorage>>
}

impl ChessApp {

    // Constructor with dependencies
    pub(super) fn new(
        console_io: Rc<RefCell<dyn ConsoleIOInterface>>,
        file_io: Rc<RefCell<dyn FileIOInterface>>
    ) -> ChessApp {
        // Create & connect children components
        let file_storage = Rc::new(RefCell::new(FileStorage::new(Rc::clone(&file_io))));
        let storage_interface = StorageProvider::get(Rc::clone(&file_storage));

        let ai_player = Arc::new(Mutex::new(AsyncAiPlayer::new()));
        let ai_player_interface = AsyncPlayerProvider::get(Arc::clone(&ai_player));

        let interactive_player_adapter = Arc::new(Mutex::new(InteractivePlayerAdapter::new()));
        let interactive_player_adapter_interface = InteractivePlayerAdapterAsyncProvider::get(Arc::clone(&interactive_player_adapter));
        let interactive_player_adapter_player_interface = AsyncPlayerProvider::get(Arc::clone(&interactive_player_adapter));

        let rules_engine = Arc::new(Mutex::new(RulesEngine::new()));
        let rules_engine_interface = RulesEngineAsyncProvider::get(Arc::clone(&rules_engine));

        let game_flow = GameFlow::new(
            &interactive_player_adapter_player_interface,
            &ai_player_interface,
            &rules_engine_interface
        );
        let game_flow_interface = GameFlowAsyncProvider::get(Arc::clone(&game_flow));
        
        let console_ui = Rc::new(RefCell::new(ConsoleUI::new(
            &Rc::clone(&console_io), 
            &Rc::clone(&storage_interface),
            &Arc::clone(&game_flow_interface),
            &Arc::clone(&interactive_player_adapter_interface)
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
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn ConsoleAppInterface>> {
        ConsoleAppProvider::get(Rc::clone(&it.borrow().console_ui))
    }
}
