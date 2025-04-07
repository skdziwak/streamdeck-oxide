# StreamDeck Oxide

A high-level framework for creating Stream Deck applications in Rust.

## Features

- Button rendering with text, icons, and custom images
- View system for organizing buttons into screens
- Navigation between views
- Event handling for button presses
- Async support for fetching state and handling actions

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
streamdeck-oxide = "0.1.0"
```

### Other dependencies

`libudev` is required for HID support. You can install it using your package
manager. `flake.nix` with a dev shell is provided for development.

## Usage

```rust
use std::sync::Arc;

use streamdeck_oxide::{
    button::RenderConfig, elgato_streamdeck, generic_array::{typenum::{U3, U5}}, md_icons, navigation::NavigationEntry, run_with_external_triggers, theme::Theme, view::{
        customizable::{ClickButton, CustomizableView, ToggleButton},
        View,
    }, ExternalTrigger
};

/// Application context for our Stream Deck app
#[derive(Debug, Clone)]
struct AppContext {
    message: String,
}

/// Navigation structure for our Stream Deck app
#[derive(Debug, Clone, Default, PartialEq)]
enum Navigation {
    #[default]
    Main,
    Settings,
    Notification(String),
}

impl NavigationEntry<U5, U3, AppContext>
    for Navigation
{
    fn get_view(
        &self,
    ) -> Result<
        Box<
            dyn View<
                U5,
                U3,
                AppContext,
                Navigation,
            >,
        >,
        Box<dyn std::error::Error>,
    > {
        match self {
            Navigation::Main => {
                let mut view = CustomizableView::default();

                // Add a toggle button
                view.set_button(
                    0,
                    0,
                    ToggleButton::new(
                        "Toggle",
                        Some(md_icons::filled::ICON_HOME),
                        |_ctx: AppContext| async move {
                            println!("Fetching toggle state");
                            // In a real app, you might fetch this from some external state
                            Ok(false)
                        },
                        |ctx: AppContext, state: bool| async move {
                            println!("Toggled state: {}", state);
                            println!("Message: {}", ctx.message);
                            Ok(())
                        },
                    ),
                )?;

                // Add a click button
                view.set_button(
                    1,
                    0,
                    ClickButton::new(
                        "Click",
                        Some(md_icons::filled::ICON_TOUCH_APP),
                        |ctx: AppContext| async move {
                            println!("Button clicked!");
                            println!("Message: {}", ctx.message);
                            Ok(())
                        },
                    ),
                )?;

                // Add a navigation button to settings
                view.set_navigation(
                    0,
                    2,
                    Navigation::Settings,
                    "Settings",
                    Some(md_icons::sharp::ICON_SETTINGS),
                )?;

                Ok(Box::new(view))
            }
            Navigation::Settings => {
                let mut view = CustomizableView::default();

                // Add some settings buttons
                view.set_button(
                    0,
                    0,
                    ClickButton::new(
                        "Option 1",
                        Some(md_icons::filled::ICON_BRIGHTNESS_5),
                        |_ctx| async move {
                            println!("Option 1 selected");
                            Ok(())
                        },
                    ),
                )?;

                view.set_button(
                    1,
                    0,
                    ClickButton::new(
                        "Option 2",
                        Some(md_icons::filled::ICON_VOLUME_UP),
                        |_ctx| async move {
                            println!("Option 2 selected");
                            Ok(())
                        },
                    ),
                )?;

                // Add a button that fails to fetch data
                view.set_button(
                    2,
                    0,
                    ClickButton::new(
                        "Fail",
                        Some(md_icons::filled::ICON_ERROR),
                        |_ctx| async move {
                            println!("This button will fail");
                            Err("Failed to fetch data".into())
                        },
                    ),
                )?;

                // Add a navigation button to go back to main
                view.set_navigation(
                    4,
                    2,
                    Navigation::Main,
                    "Back",
                    Some(md_icons::sharp::ICON_ARROW_BACK),
                )?;

                Ok(Box::new(view))
            }
            Navigation::Notification(content) => {
                let mut view = CustomizableView::default();

                view.set_button(
                    2,
                    1,
                    ClickButton::new(
                        content,
                        Some(md_icons::filled::ICON_NOTIFICATIONS),
                        |_ctx| async move { Ok(()) },
                    ),
                )?;

                view.set_navigation(
                    4,
                    2,
                    Navigation::Main,
                    "Back",
                    Some(md_icons::sharp::ICON_ARROW_BACK),
                )?;

                Ok(Box::new(view))
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("StreamDeck Example Application");
    println!("------------------------------");

    // Connect to the Stream Deck
    let hid = elgato_streamdeck::new_hidapi()?;
    let devices = elgato_streamdeck::list_devices(&hid);

    println!("Looking for Stream Deck devices...");

    let (kind, serial) = devices
        .into_iter()
        .find(|(kind, _)| *kind == elgato_streamdeck::info::Kind::Mk2)
        .ok_or("No Stream Deck found")?;

    println!("Found Stream Deck: {:?} ({})", kind, serial);

    let deck = Arc::new(elgato_streamdeck::AsyncStreamDeck::connect(
        &hid, kind, &serial,
    )?);
    println!("Connected to Stream Deck successfully!");

    // Create configuration
    let config = RenderConfig::default();
    let theme = Theme::light(); // Use light theme for this example

    // Create application context
    let context = AppContext {
        message: "Hello from StreamDeck Example!".to_string(),
    };

    println!("Starting Stream Deck application...");
    println!("Press Ctrl+C to exit");

    let (sender, receiver) = tokio::sync::mpsc::channel::<ExternalTrigger<Navigation, U5, U3, AppContext>>(1);

    // Create an endless loop sending a notification every 15 seconds
    let future = async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
            let result = sender.send(ExternalTrigger::new(Navigation::Notification("Hello!".to_string()), true)).await;
            if let Err(e) = result {
                println!("Failed to send notification: {}", e);
            } else {
                println!("Notification sent!");
            }
        }
    };

    // Run the notification loop
    tokio::spawn(future);

    // Run the application
    run_with_external_triggers::<Navigation, U5, U3, AppContext>(
        theme, config, deck, context, receiver
    )
    .await?;

    Ok(())
}
```

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
