use cygnixy_plugin_interface::{export_plugin, PluginLua};
use mlua::{Function, Lua};
use std::collections::HashMap;
use std::error::Error;
use tracing::{debug, error, info, instrument, trace, warn};

pub struct PluginLuaCygnixy;

impl Default for PluginLuaCygnixy {
    fn default() -> Self {
        Self
    }
}

impl PluginLuaCygnixy {
    pub fn new() -> Self {
        PluginLuaCygnixy
    }
}

impl PluginLua for PluginLuaCygnixy {
    fn name(&self) -> &str {
        "cygnixy"
    }

    fn on_load(&mut self) -> Result<(), Box<dyn Error>> {
        trace!("PluginLuaCygnixy loaded!");
        Ok(())
    }

    fn on_unload(&mut self) -> Result<(), Box<dyn Error>> {
        trace!("PluginLuaCygnixy unloaded!");
        Ok(())
    }

    fn get_lua_functions(&self, lua: &Lua, name: &str) -> HashMap<String, Function> {
        let mut functions = HashMap::new();

        if let Err(e) = self.register_functions(lua, name, &mut functions) {
            error!("Failed to register Lua functions: {}", e);
        }

        functions
    }
}

impl PluginLuaCygnixy {
    #[instrument(skip_all, name="lua", fields(uuid=?name))]
    fn register_functions(
        &self,
        lua: &Lua,
        name: &str,
        functions: &mut HashMap<String, Function>,
    ) -> Result<(), Box<dyn Error>> {
        // Registering the "sleep" function
        functions.insert(
            "sleep".to_string(),
            lua.create_function(|_, n: u64| {
                std::thread::sleep(std::time::Duration::from_millis(n));
                Ok(())
            })?,
        );

        // Registering the "mouse_move" function
        #[allow(unused_variables)]
        functions.insert(
            "mouse_move".to_string(),
            lua.create_function(|_, (x, y): (i32, i32)| {
                #[cfg(target_os = "windows")]
                control_craft::set_cursor_pos(x, y);
                Ok(())
            })?,
        );

        // Registering the "drag_and_drop" function
        #[allow(unused_variables)]
        functions.insert(
            "drag_and_drop".to_string(),
            lua.create_function(|_, (x, y): (i32, i32)| {
                #[cfg(target_os = "windows")]
                control_craft::drag_and_drop(x, y);
                Ok(())
            })?,
        );

        // Registering the "mouse_click_left" function
        functions.insert(
            "mouse_click_left".to_string(),
            lua.create_function(|_, ()| {
                #[cfg(target_os = "windows")]
                control_craft::click_mouse_button_left();
                Ok(())
            })?,
        );

        // Registering the "mouse_click_right" function
        functions.insert(
            "mouse_click_right".to_string(),
            lua.create_function(|_, ()| {
                #[cfg(target_os = "windows")]
                control_craft::click_mouse_button_right();
                Ok(())
            })?,
        );

        // Registering the "press_key" function
        #[allow(unused_variables)]
        functions.insert(
            "press_key".to_string(),
            lua.create_function(|_, key: u8| {
                #[cfg(target_os = "windows")]
                control_craft::press_key(key);
                Ok(())
            })?,
        );

        // Registering the "info" function
        functions.insert(
            "info".to_string(),
            lua.create_function(|_, message: String| {
                info!("{}", message);
                Ok(())
            })?,
        );

        // Registering the "warn" function
        functions.insert(
            "warn".to_string(),
            lua.create_function(|_, message: String| {
                warn!("{}", message);
                Ok(())
            })?,
        );

        // Registering the "trace" function
        functions.insert(
            "trace".to_string(),
            lua.create_function(|_, message: String| {
                trace!("{}", message);
                Ok(())
            })?,
        );

        // Registering the "debug" function
        functions.insert(
            "debug".to_string(),
            lua.create_function(|_, message: String| {
                debug!("{}", message);
                Ok(())
            })?,
        );

        // Registering the "error" function
        functions.insert(
            "error".to_string(),
            lua.create_function(|_, message: String| {
                error!("{}", message);
                Ok(())
            })?,
        );

        Ok(())
    }
}

export_plugin!(PluginLuaCygnixy);
