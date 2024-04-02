use crate::render::color::RgbColor;
use crate::render::draw2d::DrawContext2D;
use crate::render::text::FontLoader;
use crate::resolve;
use crate::resources::resources::R;
use crate::ui::attributes::Attributes;
use crate::ui::styles::Resolve;
use crate::ui::elements::child::Child;
use crate::ui::elements::{UiElement, UiElementCallbacks, UiElementState};
use crate::ui::styles::{Dimension, UiStyle};

pub struct LmaoElement {
    state: UiElementState,
    style: UiStyle,
    col: RgbColor,
    attributes: Attributes,
}

impl UiElementCallbacks for LmaoElement {
    fn init(&mut self) {
        todo!()
    }

    fn draw(&mut self, ctx: &mut DrawContext2D) {
        ctx.color(self.col.clone());
        ctx.void_rectangle(
            self.state.x,
            self.state.y,
            self.state.width,
            self.state.height,
            3
        );

        let x = self.state.content_x;
        let y = self.state.content_y;
        let height = resolve!(self, text.size) as i32;

        //println!("{x}, {y}, {height}");

        for child in self.children_mut() {
            if child.is_element() {
                let mut elem = match child {
                    Child::Element(ref mut e) => e,
                    _ => {
                        unreachable!()
                    }
                }.write();
                elem.draw(ctx);
            } else {
                let s = child.as_string();

                ctx.color(RgbColor::white());
                ctx.text(
                    false,
                    x,
                    y,
                    height,
                    s.as_str(),
                );
            }
        }
    }
}

impl UiElement for LmaoElement {
    fn new(attributes: Attributes, style: UiStyle) -> Self
    where
        Self: Sized,
    {
        Self {
            state: UiElementState::new(),
            style,
            col: RgbColor::blue(),
            attributes,
        }
    }

    fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    fn attributes_mut(&mut self) -> &Attributes {
        &mut self.attributes
    }

    fn state(&self) -> &UiElementState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut UiElementState {
        &mut self.state
    }

    fn style(&self) -> &UiStyle {
        &self.style
    }

    fn style_mut(&mut self) -> &mut UiStyle {
        &mut self.style
    }

    fn components(&self) -> (&Attributes, &UiStyle, &UiElementState) {
        (&self.attributes, &self.style, &self.state)
    }

    fn components_mut(&mut self) -> (&mut Attributes, &mut UiStyle, &mut UiElementState) {
        (&mut self.attributes, &mut self.style, &mut self.state)
    }

    fn get_size(&self, s: &str) -> Dimension<i32> {
        let font = R::fonts().get_core("default");
        let height = resolve!(self, text.size);
        let width = font.get_metrics(s).width(height as i32);
        let dim = Dimension::new(width, height as i32);
        //println!("{:?}", dim);
        dim
    }
}
