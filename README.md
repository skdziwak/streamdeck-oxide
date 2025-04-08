# StreamDeck Oxide

A high-level framework for creating Stream Deck applications in Rust.

## Features

- Button rendering with text, icons, and custom images
- View system for organizing buttons into screens
- Navigation between views
- Event handling for button presses
- Async support for fetching state and handling actions
- Plugin system for creating modular, extensible applications

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
streamdeck-oxide = "0.1.4"
```

Or if you want to use plugins:

```toml
[dependencies]
streamdeck-oxide = { version = "0.1.4", features = ["plugins"] }
```

### Other dependencies

`libudev` is required for HID support. You can install it using your package
manager. `flake.nix` with a dev shell is provided for development.

## Usage

The library provides two main approaches for building Stream Deck applications:

### Basic Usage

For simple applications, you can use the standard approach with a custom
navigation structure:

```rust
// See the full example in examples/simple/src/main.rs
use streamdeck_oxide::{
    button::RenderConfig, navigation::NavigationEntry, run_with_external_triggers,
    view::customizable::CustomizableView, // ...and more imports
};

// Define your application context
struct AppContext {
    message: String,
}

// Define your navigation structure
enum Navigation {
    Main,
    Settings,
    // ...other views
}

// Implement NavigationEntry to define your views
impl NavigationEntry<U5, U3, AppContext> for Navigation {
    fn get_view(&self) -> Result<Box<dyn View<U5, U3, AppContext, Navigation>>, Box<dyn std::error::Error>> {
        // Create and return views based on navigation state
    }
}

// Run your application
run_with_external_triggers::<Navigation, U5, U3, AppContext>(
    theme, config, deck, context, receiver
).await?;
```

For the complete example, see
[examples/simple/src/main.rs](examples/simple/src/main.rs).

### Plugin System

For more complex applications, you can use the plugin system to create modular,
extensible applications:

```rust
// See the full example in examples/plugins/src/main.rs
use streamdeck_oxide::{
    plugins::{Plugin, PluginContext, PluginNavigation}, // Plugin system imports
    // ...other imports
};

// Define your plugin
pub struct AppPlugin;

impl Plugin<U5, U3> for AppPlugin {
    fn name(&self) -> &'static str {
        "AppPlugin"
    }

    fn get_view(&self) -> Result<Box<dyn View<U5, U3, PluginContext, PluginNavigation<U5, U3>>>, Box<dyn std::error::Error>> {
        // Create and return a view for this plugin
    }
}

// Create and run your plugin-based application
let context = PluginContext::new(/* your shared context */);
run_with_external_triggers::<PluginNavigation<U5, U3>, U5, U3, PluginContext>(
    theme, config, deck, context, receiver
).await?;
```

For the complete plugin example, see
[examples/plugins/src/main.rs](examples/plugins/src/main.rs).

#### Plugin System Benefits

The plugin system offers several advantages:

- **Modular Development**: Create self-contained plugins that can be developed
  independently
- **Shared Context**: Share data between plugins using a type-safe context
  system
- **Flexible Navigation**: Navigate between plugin views seamlessly
- **Extensibility**: Add new functionality without modifying existing code
- **Sharing Plugins**: You can share your plugins with the community or use
  plugins created by others as part of your application

## Documentation

For more detailed documentation, see the
[API documentation](https://docs.rs/streamdeck-oxide).

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file
for details.

This project is using Roboto font for rendering text. The font files are
included in the `fonts` directory. Roboto is licensed under the Apache License.
See the
[LICENSE-ROBOTO](https://github.com/googlefonts/roboto-2/blob/main/LICENSE) file
for details.
