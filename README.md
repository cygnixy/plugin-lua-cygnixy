# Cygnixy Core Lua Plugin

Cygnixy Core Lua Plugin is a plugin for the **[Cygnixy framework](https://cygnixy.com)**, providing a collection of utility functions for interacting with the system, including mouse control, keyboard input, and logging features. This plugin is designed to integrate seamlessly with Lua scripts, enabling dynamic and flexible interactions within the **[Cygnixy framework](https://cygnixy.com)**.

The plugin is automatically integrated into the framework, requiring no manual setup. Once registered, its functions are immediately available for use in Lua.

## Features

* **System Interaction**:
  + Control the mouse position and simulate mouse clicks.
  + Perform drag-and-drop operations.
  + Simulate key presses.

* **Logging**:
  + Log informational, debug, and error messages to the console using `tracing`.

* **Utility Functions**:
  + Introduce delays in Lua scripts with a `sleep` function.

## Available Functions

### `sleep(milliseconds: u64)`

Pauses the execution for the specified duration.

**Example:**

```lua
cygnixy.sleep(1000) -- Pause for 1 second
```

---

### `mouse_move(x: i32, y: i32)`

Moves the mouse cursor to the specified screen coordinates.

**Example:**

```lua
cygnixy.mouse_move(500, 300)
```

---

### `drag_and_drop(x: i32, y: i32)`

Performs a drag-and-drop operation to the specified screen coordinates.

**Example:**

```lua
cygnixy.drag_and_drop(800, 600)
```

---

### `mouse_click_left()`

Simulates a left mouse button click.

**Example:**

```lua
cygnixy.mouse_click_left()
```

---

### `mouse_click_right()`

Simulates a right mouse button click.

**Example:**

```lua
cygnixy.mouse_click_right()
```

---

### `press_key(key: u8)`

Simulates a key press.

**Example:**

```lua
cygnixy.press_key(65) -- Press 'A' key
```

---

### Logging Functions

#### `info(message: String)`

Logs an informational message.

**Example:**

```lua
cygnixy.info("This is an info message")
```

#### `debug(message: String)`

Logs a debug message.

**Example:**

```lua
cygnixy.debug("This is a debug message")
```

#### `error(message: String)`

Logs an error message.

**Example:**

```lua
cygnixy.error("This is an error message")
```

---

### Join our **[Discord Community](https://discord.gg/ZjgsUrvPhD)**!

Get answers to all your questions, connect with other users, and explore the full potential of the **[Cygnixy framework](https://cygnixy.com)**.

---

## License

This project is licensed under the MIT License.
