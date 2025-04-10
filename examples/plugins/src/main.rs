use std::{any::{Any, TypeId}, collections::BTreeMap, sync::Arc};

use streamdeck_oxide::{
    button::RenderConfig, elgato_streamdeck, generic_array::typenum::{U3, U5}, md_icons, plugins::{Plugin, PluginContext, PluginNavigation}, run_with_external_triggers, theme::Theme, view::{
        customizable::{ClickButton, CustomizableView},
        View,
    }, ExternalTrigger
};

pub struct AppPlugin;
pub struct AppPlugin2;
pub struct AppPluginContext {
    pub message: String,
}

#[async_trait::async_trait]
impl Plugin<U5, U3> for AppPlugin {
    fn name(&self) -> &'static str {
        "AppPlugin"
    }

    async fn get_view(&self, context: PluginContext) -> Result<Box<dyn View<U5, U3, PluginContext, PluginNavigation<U5, U3>>>, Box<dyn std::error::Error>> {
        let mut view = CustomizableView::new();
        view.set_button(
            0,
            0,
            ClickButton::new(
                "Click Me",
                Some(md_icons::filled::ICON_HOME),
                |context: PluginContext| async move {
                    let context = context.get_context::<AppPluginContext>();
                    if let Some(context) = context.await {
                        println!("Button clicked! Message: {}", &context.message);
                    } else {
                        println!("No context found");
                    }
                    Ok(())
                },
            ),
        )?;

        view.set_navigation(0, 2, PluginNavigation::<U5, U3>::new(AppPlugin2), "View 2", Some(md_icons::filled::ICON_CHECK))?;

        Ok(Box::new(view))
    }
}

#[async_trait::async_trait]
impl Plugin<U5, U3> for AppPlugin2 {
    fn name(&self) -> &'static str {
        "AppPlugin2"
    }

    async fn get_view(&self, context: PluginContext) -> Result<Box<dyn View<U5, U3, PluginContext, PluginNavigation<U5, U3>>>, Box<dyn std::error::Error>> {
        let mut view = CustomizableView::new();
        view.set_button(
            0,
            0,
            ClickButton::new(
                "Second View",
                Some(md_icons::filled::ICON_CHECK),
                |context: PluginContext| async move {
                    let context = context.get_context::<AppPluginContext>();
                    if let Some(context) = context.await {
                        println!("Button clicked! Message: {}", &context.message);
                    } else {
                        println!("No context found");
                    }
                    Ok(())
                },
            ),
        )?;

        view.set_navigation(0, 2, PluginNavigation::<U5, U3>::new(AppPlugin), "View 1", Some(md_icons::filled::ICON_CHECK))?;

        Ok(Box::new(view))
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


    println!("Starting Stream Deck application...");
    println!("Press Ctrl+C to exit");

    let (sender, receiver) = tokio::sync::mpsc::channel::<ExternalTrigger<PluginNavigation<U5, U3>, U5, U3, PluginContext>>(1);

    sender.send(ExternalTrigger::new(
        PluginNavigation::<U5, U3>::new(AppPlugin),
        true
    )).await?;

    let app_plugin_context = AppPluginContext {
        message: "Hello, Stream Deck!".to_string(),
    };
    let context = PluginContext::new(
        BTreeMap::from([
            (TypeId::of::<AppPluginContext>(), Box::new(Arc::new(app_plugin_context)) as Box<dyn Any + Send + Sync>)
        ])
    );

    // Run the application
    run_with_external_triggers::<PluginNavigation<U5, U3>, U5, U3, PluginContext>(
        theme, config, deck, context, receiver
    )
    .await?;

    Ok(())
}
