# StreamDeck Plugins Example

This example demonstrates how to use the plugin system in the `streamdeck-oxide` library to create modular Stream Deck applications.

## Features

This example demonstrates:
- Creating custom plugins that implement the `Plugin` trait
- Sharing data between plugins using `PluginContext`
- Navigation between different plugin views
- Dynamic dispatch for plugin functionality

## Running the Example

Make sure you have a Stream Deck connected to your computer, then run:

```bash
cargo run
```

## Structure

The example consists of two plugins:
1. **AppPlugin** - The main plugin with a button and navigation to the second plugin
2. **AppPlugin2** - A secondary plugin with its own view and navigation back to the first plugin

## Plugin System Overview

The plugin system allows you to:

1. **Create modular components**: Each plugin is a self-contained module that can be developed independently.
2. **Share context between plugins**: The `PluginContext` allows plugins to access shared data.
3. **Navigate between plugin views**: Plugins can navigate to other plugins' views.
4. **Build libraries of plugins**: Plugins can be compiled into separate libraries and loaded dynamically.

## Code Explanation

### Plugin Implementation

Each plugin implements the `Plugin` trait:

```rust
impl Plugin<U5, U3> for AppPlugin {
    fn name(&self) -> &'static str {
        "AppPlugin"
    }

    fn get_view(&self) -> Result<Box<dyn View<U5, U3, PluginContext, PluginNavigation<U5, U3>>>, Box<dyn std::error::Error>> {
        // Create and return a view
    }
}
```

### Plugin Context

The `PluginContext` allows sharing data between plugins:

```rust
// Define a custom context type
pub struct AppPluginContext {
    pub message: String,
}

// Create and initialize the context
let app_plugin_context = AppPluginContext {
    message: "Hello, Stream Deck!".to_string(),
};

// Add the context to the PluginContext
let context = PluginContext::new(
    BTreeMap::from([
        (TypeId::of::<AppPluginContext>(), Box::new(Arc::new(app_plugin_context)) as Box<dyn Any + Send + Sync>)
    ])
);
```

### Accessing Context in Plugins

Plugins can access the shared context:

```rust
|context: PluginContext| async move {
    let context = context.get_context::<AppPluginContext>();
    if let Some(context) = context.await {
        println!("Button clicked! Message: {}", &context.message);
    } else {
        println!("No context found");
    }
    Ok(())
}
```

### Navigation Between Plugins

Plugins can navigate to other plugins:

```rust
view.set_navigation(0, 2, PluginNavigation::<U5, U3>::new(AppPlugin2), "View 2", Some(md_icons::filled::ICON_CHECK))?;
```

## Customizing

You can customize this example by:
- Creating additional plugins
- Adding more functionality to existing plugins
- Extending the shared context with more data
- Creating more complex navigation flows between plugins

## Learn More

For more information, see the [StreamDeck library documentation](https://docs.rs/streamdeck-oxide).