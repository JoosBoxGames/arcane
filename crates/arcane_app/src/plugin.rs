use crate::App;

pub trait Plugin: Send + Sync + 'static {
    /// Configures the [`App`] to which this plugin is added.
    fn build(&self, app: &mut App);

    /// Configures a name for the [`Plugin`] which is primarily used for debugging.
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}
