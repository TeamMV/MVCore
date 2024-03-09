pub mod child;

use std::any::{Any, TypeId};
use crate::resolve;
use crate::ui::styles::{Origin, Position, ResCon, Style, UiValue};
use mvutils::unsafe_utils::Unsafe;
use mvutils::utils::{Recover, RwArc, TetrahedronOp};
use std::sync::{Arc, RwLock};
use crate::ui::attributes::Attributes;
use crate::ui::elements::child::Child;

pub trait UiElementCallbacks {
    fn init(&mut self);

    fn draw(&mut self, ctx: &mut DrawContext2D);
}

pub trait UiElement: UiElementCallbacks {
    fn new(attributes: Attributes, style: Style) -> Self where Self: Sized;

    fn add_child(&mut self, child: Child);
}