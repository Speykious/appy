use crate::*;
use std::rc::Rc;

#[derive(Default, Clone)]
pub enum Dim {
    #[default]
    None,
    Pc(f32),
    Px(f32),
    Dp(f32)
}

impl Dim {
    pub fn is_some(&self) -> bool {
        !matches!(*self, Dim::None)
    }

    pub fn to_px(&self, max: f32, pixel_ratio: f32) -> f32 {
        match *self {
            Dim::Px(v) => v,
            Dim::Pc(v) => max * v / 100.0,
            Dim::Dp(v) => v * pixel_ratio,
            Dim::None => panic!("can't get px from none"),
        }
    }

    pub fn compute_span(max: f32, pr: f32, start: Dim, size: Dim, end: Dim) -> (f32, f32) {
        let c = if start.is_some() { 1 << 2 } else { 0 }
            + if size.is_some() { 1 << 1 } else { 0 }
            + if end.is_some() { 1 << 0 } else { 0 };

        match c {
            0b000 => (0.0, max),
            0b001 => (0.0, max - end.to_px(max,pr)),
            0b010 => ((max - size.to_px(max,pr)) / 2.0, size.to_px(max,pr)),
            0b011 => (max - size.to_px(max,pr) - end.to_px(max,pr), size.to_px(max,pr)),
            0b100 => (start.to_px(max,pr), max - start.to_px(max,pr)),
            0b101 => (start.to_px(max,pr), max - start.to_px(max,pr) - end.to_px(max,pr)),
            0b110 => (start.to_px(max,pr), size.to_px(max,pr)),
            0b111 => panic!("over constrained"),
            _ => panic!("bad constraints"),
        }
    }
}

impl Dim {}

#[derive(Default)]
pub struct Blk {
    pub left: Dim,
    pub top: Dim,
    pub width: Dim,
    pub height: Dim,
    pub bottom: Dim,
    pub right: Dim,
}

#[function_component]
pub fn blk(p: Blk, children: Elements) -> Elements {
    let instance_ref = use_context::<AppContext>();
    let mut instance = instance_ref.borrow_mut();

    let old_rect = instance.rect.clone();
    let h = Dim::compute_span(old_rect.w as f32, instance.pixel_ratio, p.left, p.width, p.right);
    let v = Dim::compute_span(old_rect.h as f32, instance.pixel_ratio,  p.top, p.height, p.bottom);

    instance.rect = instance
        .rect
        .abs(h.0 as i32, v.0 as i32, h.1 as i32, v.1 as i32);

    use_post_render(Rc::new(with_clone!([instance_ref], move || {
        let mut instance = instance_ref.borrow_mut();
        instance.rect = old_rect.clone();
    })));

    children
}
