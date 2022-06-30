use super::view::View;

// enums fileds written with small letters to be kind of comprehensive with vim style commands
#[allow(non_camel_case_types)]
pub enum CommandsOpCode {
    move_cursor_up,
    move_cursor_down,
    move_cursor_left,
    move_cursor_right,
}

pub struct CommandExecutor {}

impl CommandExecutor {
    pub fn create() -> CommandExecutor {
        CommandExecutor {}
    }

    fn empty_cmd_execution(&self, view: &mut View) {}

    fn move_cursor_up(&self, view: &mut View) {
        view.move_cursor(super::view::CursorMoves::Up);
    }

    fn move_cursor_down(&self, view: &mut View) {
        view.move_cursor(super::view::CursorMoves::Down);
    }

    fn move_cursor_left(&self, view: &mut View) {
        view.move_cursor(super::view::CursorMoves::Left);
    }

    fn move_cursor_right(&self, view: &mut View) {
        view.move_cursor(super::view::CursorMoves::Right);
    }
    pub fn execute_cmd(&self, view: &mut View, op_code: CommandsOpCode) {
        type Func = fn(&CommandExecutor, &mut View);
        let func: Func;
        match op_code {
            CommandsOpCode::move_cursor_up => func = CommandExecutor::move_cursor_up,
            CommandsOpCode::move_cursor_down => func = CommandExecutor::move_cursor_down,
            CommandsOpCode::move_cursor_left => func = CommandExecutor::move_cursor_left,
            CommandsOpCode::move_cursor_right => func = CommandExecutor::move_cursor_right,
            _ => func = CommandExecutor::empty_cmd_execution,
        }
        func(self, view);
    }
}
