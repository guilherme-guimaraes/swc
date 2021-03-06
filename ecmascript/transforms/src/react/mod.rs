pub use self::{
    display_name::display_name,
    jsx::{jsx, Options},
    jsx_self::jsx_self,
    jsx_src::jsx_src,
};
use crate::pass::Pass;
use std::sync::Arc;
use swc_common::SourceMap;

mod display_name;
mod jsx;
mod jsx_self;
mod jsx_src;

/// `@babel/preset-react`
///
/// Preset for all React plugins.
pub fn react(cm: Arc<SourceMap>, options: Options) -> impl Pass + Clone {
    let Options { development, .. } = options;

    chain!(
        jsx(cm.clone(), options),
        display_name(),
        jsx_src(development, cm),
        jsx_self(development)
    )
}
