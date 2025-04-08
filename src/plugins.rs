use std::{
    any::{Any, TypeId},
    collections::BTreeMap,
    sync::Arc,
};

use generic_array::ArrayLength;

use crate::{view::customizable::CustomizableView, NavigationEntry, View};

type GetViewResult<W, H, C, N> = Result<Box<dyn View<W, H, C, N>>, Box<dyn std::error::Error>>;

pub trait Plugin<W, H>: Send + Sync + 'static
where
    W: ArrayLength,
    H: ArrayLength,
{
    fn name(&self) -> &'static str;
    fn get_view(&self) -> GetViewResult<W, H, PluginContext, PluginNavigation<W, H>>;
}

struct DefaultPlugin;

impl <W, H> Plugin<W, H> for DefaultPlugin where W: ArrayLength, H: ArrayLength {
    fn name(&self) -> &'static str {
        "DefaultPlugin"
    }

    fn get_view(&self) -> GetViewResult<W, H, PluginContext, PluginNavigation<W, H>> {
        Ok(Box::new(CustomizableView::new()))
    }
}

#[derive(Clone)]
pub struct PluginNavigation<W: ArrayLength, H: ArrayLength> {
    pub(crate) plugin: Arc<Box<dyn Plugin<W, H>>>,
}

impl <W: ArrayLength, H: ArrayLength> PluginNavigation<W, H> {
    pub fn new(plugin: impl Plugin<W, H> + Send + Sync + 'static) -> Self {
        Self {
            plugin: Arc::new(Box::new(plugin)),
        }
    }
}

impl <W, H> Default for PluginNavigation<W, H> where W: ArrayLength, H: ArrayLength {
    fn default() -> Self {
        Self { plugin: Arc::new(Box::new(DefaultPlugin)) }
    }
}

#[derive(Default, Clone)]
pub struct PluginContext {
    pub(crate) contexts: Arc<BTreeMap<TypeId, Box<dyn Any + Send + Sync + 'static>>>,
}

impl PluginContext {
    pub fn new(tree: BTreeMap<TypeId, Box<dyn Any + Send + Sync + 'static>>) -> Self {
        Self {
            contexts: Arc::new(tree),
        }
    }

    pub async fn get_context<T: Any>(&self) -> Option<Arc<T>> {
        if let Some(context) = self.contexts.get(&TypeId::of::<T>()) {
            if let Some(context) = context.downcast_ref::<Arc<T>>() {
                return Some(context.clone());
            }
        }
        None
    }
}

impl<W, H> PartialEq for PluginNavigation<W, H>
where
    W: ArrayLength,
    H: ArrayLength,
{
    fn eq(&self, other: &Self) -> bool {
        self.plugin.name() == other.plugin.name()
    }
}

impl<W, H> NavigationEntry<W, H, PluginContext> for PluginNavigation<W, H>
where
    W: ArrayLength,
    H: ArrayLength,
{
    fn get_view(
        &self,
    ) -> Result<Box<dyn View<W, H, PluginContext, Self>>, Box<dyn std::error::Error>> {
         self.plugin.get_view()
    }
}
