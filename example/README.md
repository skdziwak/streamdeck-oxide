# StreamDeck Example Application

This is a simple example application demonstrating how to use the `streamdeck-oxide` library.

## Features

This example demonstrates:
- Creating a Stream Deck application with multiple views
- Using toggle buttons and click buttons
- Navigation between views
- Custom application context

## Running the Example

Make sure you have a Stream Deck connected to your computer, then run:

```bash
cargo run
```

## Structure

The example consists of two views:
1. **Main View** - Contains a toggle button, a click button, and a navigation button to the settings view
2. **Settings View** - Contains two option buttons and a navigation button back to the main view

## Code Overview

The main components of the example are:

- `AppContext` - A simple struct containing application data
- `Navigation` - An enum defining the different views in the application
- `NavigationEntry` implementation - Defines how to create views for each navigation entry
- Main function - Connects to the Stream Deck and runs the application

## Customizing

You can customize this example by:
- Adding more views to the `Navigation` enum
- Adding more buttons to the views
- Changing the button actions
- Modifying the application context

## Learn More

For more information, see the [StreamDeck library documentation](https://docs.rs/streamdeck-oxide).