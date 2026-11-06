use gpui::{
    App, ObjectFit, Rgba, SharedString, StyleRefinement, Window, img, prelude::*, rgb, svg,
};

const FALLBACK_ICON: &str = "icons/globe.svg";
const FALLBACK_COLOR: u32 = 0xF8F8F8;

#[derive(Clone, IntoElement)]
pub struct Favicon {
    style: StyleRefinement,
    source: SharedString,
    fallback: SharedString,
}

impl Favicon {
    pub fn new(source: impl Into<SharedString>) -> Self {
        Self {
            style: StyleRefinement::default(),
            source: source.into(),
            fallback: FALLBACK_ICON.into(),
        }
    }

    pub fn color(mut self, color: Rgba) -> Self {
        self.style.text.color = Some(color.into());
        self
    }

    pub fn fallback(mut self, fallback: SharedString) -> Self {
        self.fallback = fallback;
        self
    }

    pub fn is_remote(&self) -> bool {
        self.source.starts_with("http://") || self.source.starts_with("https://")
    }
}

impl Styled for Favicon {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Favicon {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let remote = self.is_remote();
        let style = self.style;

        if remote {
            let (fallback, fallback_style) = (self.fallback, style.clone());

            let mut image = img(self.source)
                .object_fit(ObjectFit::Contain)
                .with_fallback(move || {
                    let mut icon = svg().path(fallback.clone());
                    *icon.style() = fallback_style.clone().text_color(rgb(FALLBACK_COLOR));
                    icon.into_any_element()
                });
            *image.style() = style;
            image.into_any_element()
        } else {
            let mut icon = svg().path(self.source.clone());
            *icon.style() = style.clone();
            icon.into_any_element()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_remote_sources() {
        assert!(Favicon::new("https://a.com/f.ico").is_remote());
        assert!(Favicon::new("http://a.com/f.ico").is_remote());
        assert!(!Favicon::new("icons/globe.svg").is_remote());
        assert!(!Favicon::new("").is_remote());
    }
}
