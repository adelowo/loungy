use gpui::*;
use serde::Deserialize;

use crate::{
    commands::root::list::RootBuilder,
    icon::Icon,
    list::{Accessory, Img, ImgMask, ImgSize, ImgSource, Item, List, ListItem},
    nucleo::fuzzy_match,
    query::{TextEvent, TextInput},
    theme,
};

pub struct StateItem {
    pub query: TextInput,
    pub view: AnyView,
    pub actions: ActionsModel,
}

impl StateItem {
    pub fn init(view: impl StateView, cx: &mut WindowContext) -> Self {
        let actions = ActionsModel::init(cx);
        let query = TextInput::new(&actions, cx);
        //let actions_clone = actions.clone();
        cx.subscribe(&query.view, move |_, event, cx| match event {
            TextEvent::Blur => {
                // if !actions_clone.inner.read(cx).show {
                //     cx.hide();
                // };
            }
            TextEvent::KeyDown(ev) => match ev.keystroke.key.as_str() {
                "escape" => {
                    cx.hide();
                }
                _ => {}
            },
            TextEvent::Back => {
                cx.update_global::<StateModel, _>(|this, cx| {
                    this.pop(cx);
                });
            }
            _ => {}
        })
        .detach();
        let view = view.build(&query, &actions, cx);
        Self {
            query,
            view,
            actions,
        }
    }
}

pub trait StateView {
    fn build(&self, query: &TextInput, actions: &ActionsModel, cx: &mut WindowContext) -> AnyView;
}

pub struct State {
    pub stack: Vec<StateItem>,
}

#[derive(Clone)]
pub struct StateModel {
    pub inner: Model<State>,
}

impl StateModel {
    pub fn init(cx: &mut WindowContext) -> Self {
        let item = StateItem::init(RootBuilder {}, cx);
        let state = cx.new_model(|_| State { stack: vec![item] });
        Self { inner: state }
    }
    pub fn pop(&self, cx: &mut WindowContext) {
        self.inner.update(cx, |model, cx| {
            if model.stack.len() > 1 {
                model.stack.pop();
                cx.notify();
            };
        });
    }
    pub fn push(&self, view: impl StateView, cx: &mut WindowContext) {
        let item = StateItem::init(view, cx);
        self.inner.update(cx, |model, cx| {
            model.stack.push(item);
            cx.notify();
        });
    }
}

impl Global for StateModel {}

// Actions

pub trait CloneableFn: Fn(&mut WindowContext) -> () {
    fn clone_box<'a>(&self) -> Box<dyn 'a + CloneableFn>
    where
        Self: 'a;
}

impl<F> CloneableFn for F
where
    F: Fn(&mut WindowContext) -> () + Clone,
{
    fn clone_box<'a>(&self) -> Box<dyn 'a + CloneableFn>
    where
        Self: 'a,
    {
        Box::new(self.clone())
    }
}

impl<'a> Clone for Box<dyn 'a + CloneableFn> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}

#[derive(Clone, IntoElement, Deserialize)]
pub struct Shortcut {
    inner: Keystroke,
}

impl Shortcut {
    pub fn simple(key: impl ToString) -> Self {
        Self {
            inner: Keystroke {
                modifiers: Modifiers::default(),
                key: key.to_string(),
                ime_key: None,
            },
        }
    }
    pub fn cmd(key: impl ToString) -> Self {
        Self {
            inner: Keystroke {
                modifiers: Modifiers {
                    command: true,
                    ..Modifiers::default()
                },
                key: key.to_string(),
                ime_key: None,
            },
        }
    }
    pub fn new(keystroke: Keystroke) -> Self {
        Self { inner: keystroke }
    }
}

fn key_icon(el: Div, icon: Icon) -> Div {
    el.child(
        div()
            .child(Img::new(
                ImgSource::Icon(icon),
                ImgMask::Rounded,
                ImgSize::Small,
            ))
            .ml_0p5(),
    )
}

impl RenderOnce for Shortcut {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<theme::Theme>();
        let mut el = div().flex().items_center();
        let shortcut = self.inner;
        if shortcut.modifiers.control {
            el = key_icon(el, Icon::ChevronUp);
        }
        if shortcut.modifiers.alt {
            el = key_icon(el, Icon::Option);
        }
        if shortcut.modifiers.shift {
            el = key_icon(el, Icon::ArrowBigUp);
        }
        if shortcut.modifiers.command {
            el = key_icon(el, Icon::Command);
        }
        match shortcut.key.as_str() {
            "enter" => {
                el = key_icon(el, Icon::CornerDownLeft);
            }
            "backspace" => {
                el = key_icon(el, Icon::Delete);
            }
            "delete" => {
                el = key_icon(el, Icon::Delete);
            }
            "escape" => {
                el = key_icon(el, Icon::ArrowUpRightFromSquare);
            }
            "tab" => {
                el = key_icon(el, Icon::ArrowRightToLine);
            }
            "space" => {
                el = key_icon(el, Icon::Space);
            }
            "up" => {
                el = key_icon(el, Icon::ArrowUp);
            }
            "down" => {
                el = key_icon(el, Icon::ArrowDown);
            }
            "left" => {
                el = key_icon(el, Icon::ArrowLeft);
            }
            "right" => {
                el = key_icon(el, Icon::ArrowRight);
            }
            _ => {
                el = el.child(
                    div()
                        .size_5()
                        .p_1()
                        .rounded_md()
                        .bg(theme.surface0)
                        .text_color(theme.text)
                        .font_weight(FontWeight::MEDIUM)
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(shortcut.ime_key.unwrap_or(shortcut.key).to_uppercase())
                        .ml_0p5(),
                )
            }
        }
        el
    }
}

#[derive(Clone, IntoElement)]
pub struct Action {
    pub label: String,
    pub shortcut: Option<Shortcut>,
    pub image: Img,
    pub action: Box<dyn CloneableFn>,
    pub hide: bool,
}

impl RenderOnce for Action {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let shortcut = if let Some(shortcut) = self.shortcut {
            div().child(shortcut)
        } else {
            div()
        };
        div()
            .ml_auto()
            .child(div().child(self.label).mr_2())
            .flex()
            .items_center()
            .justify_between()
            .child(shortcut)
    }
}

impl Action {
    pub fn new(
        image: Img,
        label: impl ToString,
        shortcut: Option<Shortcut>,
        action: Box<dyn CloneableFn>,
        hide: bool,
    ) -> Self {
        Self {
            label: label.to_string(),
            shortcut,
            action,
            image,
            hide,
        }
    }
}

pub struct Actions {
    global: Model<Vec<Action>>,
    local: Model<Vec<Action>>,
    show: bool,
    query: Option<TextInput>,
    list: Option<View<List>>,
    toggle: Box<dyn CloneableFn>,
}

impl Actions {
    fn combined(&self, cx: &mut ViewContext<Self>) -> Vec<Action> {
        let mut combined = self.local.read(cx).clone();
        combined.append(&mut self.global.read(cx).clone());
        if let Some(action) = combined.get_mut(0) {
            action.shortcut = Some(Shortcut::simple("enter"));
            combined.push(Action::new(
                Img::list_icon(Icon::BookOpen),
                "Actions",
                Some(Shortcut::cmd("k")),
                self.toggle.clone(),
                true,
            ))
        }
        combined
    }
    fn popup(&mut self, cx: &mut ViewContext<Self>) -> Div {
        if !self.show {
            return div();
        }
        let theme = cx.global::<theme::Theme>();
        let query = self.query.clone().unwrap();
        div()
            .absolute()
            .bottom_10()
            .right_0()
            .z_index(1000)
            .w_80()
            .max_h_48()
            .bg(theme.base)
            .rounded_xl()
            .border_1()
            .border_color(theme.crust)
            .shadow_lg()
            .flex()
            .flex_col()
            .child(div().child(self.list.clone().unwrap()).p_2())
            .child(
                div()
                    .child(query)
                    .mt_auto()
                    .px_2()
                    .border_t_1()
                    .border_color(theme.mantle)
                    .text_base(),
            )
    }
    fn list_actions(&self, cx: &mut ViewContext<Self>) {
        let text = self.query.clone().unwrap().view.read(cx).text.clone();
        let items: Vec<Item> = self
            .combined(cx)
            .into_iter()
            .filter_map(|item| {
                if item.hide {
                    return None;
                }
                let action = item.clone();
                let accessories = if let Some(shortcut) = item.shortcut {
                    vec![Accessory::shortcut(shortcut)]
                } else {
                    vec![]
                };
                Some(Item::new(
                    vec![item.label.clone()],
                    cx.new_view(|_| {
                        ListItem::new(Some(action.image.clone()), item.label, None, accessories)
                    })
                    .into(),
                    None,
                    vec![action],
                    None,
                ))
            })
            .collect();

        let items = fuzzy_match(&text, items, false);
        self.list.clone().unwrap().update(cx, |this, cx| {
            this.items = items;
            cx.notify();
        });
    }
}

impl Render for Actions {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let combined = self.combined(cx).clone();
        let theme = cx.global::<theme::Theme>();
        if let Some(action) = combined.get(0) {
            let open = combined.last().unwrap().clone();
            div()
                .ml_auto()
                .flex()
                .items_center()
                .font_weight(FontWeight::SEMIBOLD)
                .child(div().child(action.clone()).text_color(theme.text))
                .child(div().h_2_3().w(Pixels(2.0)).bg(theme.surface0).mx_2())
                .child(open)
                .child(self.popup(cx))
        } else {
            div()
        }
    }
}

#[derive(Clone)]
pub struct ActionsModel {
    pub inner: View<Actions>,
}

impl ActionsModel {
    pub fn init(cx: &mut WindowContext) -> Self {
        let global = cx.new_model(|_| Vec::new());
        let local = cx.new_model(|_| Vec::new());
        let inner = cx.new_view(|_| Actions {
            global,
            local,
            show: false,
            query: None,
            list: None,
            toggle: Box::new(|_| {}),
        });
        let clone = inner.clone();
        let toggle: Box<dyn CloneableFn> = Box::new(move |cx| {
            clone.update(cx, |model, cx| {
                model.show = !model.show;
                cx.notify();
            });
        });

        let model = Self {
            inner: inner.clone(),
        };
        let query = TextInput::new(&model, cx);
        let list = List::new(&query, None, cx);
        inner.update(cx, |this, cx| {
            this.toggle = toggle;
            this.list = Some(list);
            cx.subscribe(&query.view, move |this, _, event, cx| {
                match event {
                    TextEvent::Blur | TextEvent::Back => {
                        this.show = false;
                        cx.notify();
                    }
                    TextEvent::KeyDown(ev) => match ev.keystroke.key.as_str() {
                        "escape" => {
                            this.show = false;
                            cx.notify();
                        }
                        _ => {}
                    },
                    TextEvent::Input { text: _ } => {
                        this.list_actions(cx);
                    } //_ => {}
                }
                cx.notify();
            })
            .detach();

            this.query = Some(query);

            cx.notify();
        });
        model
    }
    pub fn update_global(&self, actions: Vec<Action>, cx: &mut WindowContext) {
        self.inner.update(cx, |model, cx| {
            model.global.update(cx, |this, cx| {
                *this = actions;
                cx.notify();
            });
            model.list_actions(cx);
        });
    }
    pub fn update_local(&self, actions: Vec<Action>, cx: &mut WindowContext) {
        self.inner.update(cx, |model, cx| {
            model.local.update(cx, |this, cx| {
                *this = actions;
                cx.notify();
            });
            model.list_actions(cx);
        });
    }
    pub fn get(&self, cx: &mut WindowContext) -> Vec<Action> {
        let mut outer: Option<Vec<Action>> = None;
        self.inner.update(cx, |model, cx| {
            outer = Some(model.combined(cx));
        });
        outer.unwrap()
    }
    pub fn check(&self, keystroke: &Keystroke, cx: &mut WindowContext) -> Option<Action> {
        let actions = &self.get(cx);
        for action in actions {
            if let Some(shortcut) = &action.shortcut {
                if shortcut.inner.eq(keystroke) {
                    return Some(action.clone());
                }
            }
        }
        None
    }
}
