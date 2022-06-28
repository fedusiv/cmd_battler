use super::{cursor::Cursor, view::View};
use std::ptr;

// enums fileds written with small letters to be kind of comprehensive with vim style commands
#[allow(non_camel_case_types)]
pub enum CommandsOpCode {
    move_cursor_up,
    move_cursor_down,
    move_cursor_left,
    move_cursor_right,
}

pub struct CommandExecutor {
    view: *const View,
}

impl CommandExecutor {
    pub fn create() -> CommandExecutor {
        CommandExecutor { view: ptr::null() }
    }

    pub fn init(&mut self, view: *const View) {
        self.view = view;
    }

    fn empty_cmd_execution(&self) {}
    fn move_cursor_right(&self) {}

    pub fn execute_cmd(&self, op_code: CommandsOpCode) {
        type Func = fn(&CommandExecutor);
        let func: Func;
        match op_code {
            CommandsOpCode::move_cursor_right => func = CommandExecutor::move_cursor_right,
            _ => func = CommandExecutor::empty_cmd_execution,
        }
        func(self);
    }
}
