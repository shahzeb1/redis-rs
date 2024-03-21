use wasm_bindgen::prelude::*;

use crate::{
    actions::{parse_action_from_user_string, Action, ActionTrait, DataType},
    create_data_container,
};

#[wasm_bindgen]
pub struct WasmRunnerContainer {
    data: DataType,
}

// The WASM zone:
// TODO: Honestly this is an area that could use some love.
// I made the ShellRunner trait so that other runners (WASM and shell) could
// reuse the same methods. However, wasm_bindgen doesn't seem to like traits.
// There's a Github issue about this, that's been open for a while.
// Git: https://github.com/rustwasm/wasm-bindgen/issues/2073
#[wasm_bindgen]
impl WasmRunnerContainer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmRunnerContainer {
        let data = create_data_container();
        return WasmRunnerContainer { data };
    }

    pub fn run(&mut self, input: &str) -> String {
        let action_result = parse_action_from_user_string(input);

        match action_result {
            Ok(action) => match action {
                Action::GetAction(get_item) => {
                    let key = get_item.0;
                    match self.data.get(&key) {
                        Some(data) => {
                            return format!("\"{}\"", data).to_string();
                        }
                        None => {
                            return "(nil)".to_string();
                        }
                    }
                }
                Action::SetAction(set_item) => {
                    set_item.execute(&mut self.data);
                    return "OK".to_string();
                }
                Action::IncrAction(incr_item) => {
                    incr_item.execute(&mut self.data);
                    let key = incr_item.0;
                    match self.data.get(&key) {
                        Some(data) => {
                            return format!("\"{}\"", data).to_string();
                        }
                        None => {
                            return "(nil)".to_string();
                        }
                    }
                }
            },

            Err(_) => "(error) I'm sorry, I don't recognize that command.".to_string(),
        }
    }
}
