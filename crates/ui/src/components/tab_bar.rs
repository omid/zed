use gpui::{red, AnyElement, ScrollHandle};
use smallvec::SmallVec;

use crate::prelude::*;
use gpui::deferred;
use gpui::MouseButton;
use gpui::MouseDownEvent;
use gpui::MouseUpEvent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(IntoElement)]
pub struct TabBar {
    id: ElementId,
    start_children: SmallVec<[AnyElement; 2]>,
    children: SmallVec<[AnyElement; 2]>,
    end_children: SmallVec<[AnyElement; 2]>,
    scroll_handle: Option<ScrollHandle>,
    position: TabBarPosition,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum TabBarPosition {
    #[default]
    Top,
    Right,
}

impl TabBar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            start_children: SmallVec::new(),
            children: SmallVec::new(),
            end_children: SmallVec::new(),
            scroll_handle: None,
            position: TabBarPosition::Top,
        }
    }

    pub fn track_scroll(mut self, scroll_handle: ScrollHandle) -> Self {
        self.scroll_handle = Some(scroll_handle);
        self
    }

    pub fn start_children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]> {
        &mut self.start_children
    }

    pub fn start_child(mut self, start_child: impl IntoElement) -> Self
    where
        Self: Sized,
    {
        self.start_children_mut()
            .push(start_child.into_element().into_any());
        self
    }

    pub fn start_children(
        mut self,
        start_children: impl IntoIterator<Item = impl IntoElement>,
    ) -> Self
    where
        Self: Sized,
    {
        self.start_children_mut().extend(
            start_children
                .into_iter()
                .map(|child| child.into_any_element()),
        );
        self
    }

    pub fn end_children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]> {
        &mut self.end_children
    }

    pub fn end_child(mut self, end_child: impl IntoElement) -> Self
    where
        Self: Sized,
    {
        self.end_children_mut()
            .push(end_child.into_element().into_any());
        self
    }

    pub fn end_children(mut self, end_children: impl IntoIterator<Item = impl IntoElement>) -> Self
    where
        Self: Sized,
    {
        self.end_children_mut().extend(
            end_children
                .into_iter()
                .map(|child| child.into_any_element()),
        );
        self
    }

    pub fn position(mut self, position: TabBarPosition) -> Self {
        self.position = position;
        self
    }
}

impl ParentElement for TabBar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

#[derive(Clone, Render)]
struct DraggedTabBar(TabBarPosition);

impl RenderOnce for TabBar {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let tab_buttons_container = match self.position {
            TabBarPosition::Top => div().h_full(),
            TabBarPosition::Right => v_flex().w_full(),
        };
        let tabs = match self.position {
            TabBarPosition::Top => h_flex(),
            TabBarPosition::Right => v_flex(),
        };
        let tab_bar = match self.position {
            TabBarPosition::Top => div().w_full(),
            TabBarPosition::Right => div().h_full(),
        };

        let mut v_tab_bar_buttons = h_flex()
            .justify_between()
            .border_b_1()
            .border_l_1()
            .border_color(cx.theme().colors().border)
            .flex_grow()
            .child(h_flex().children(self.start_children));

        if self.position == TabBarPosition::Right && !self.end_children.is_empty() {
            v_tab_bar_buttons = v_tab_bar_buttons.children(self.end_children);
        }

        // let create_resize_handle = || {
        //     let handle = div()
        //         .id("resize-handle")
        //         .on_drag(DraggedTabBar(self.position), |dock, cx| {
        //             cx.stop_propagation();
        //             cx.new_view(|_| dock.clone())
        //         })
        //         .on_mouse_down(
        //             MouseButton::Left,
        //             cx.listener(|_, _: &MouseDownEvent, cx| {
        //                 cx.stop_propagation();
        //             }),
        //         )
        //         .on_mouse_up(
        //             MouseButton::Left,
        //             cx.listener(|v, e: &MouseUpEvent, cx| {
        //                 if e.click_count == 2 {
        //                     v.resize_active_panel(None, cx);
        //                     cx.stop_propagation();
        //                 }
        //             }),
        //         )
        //         .occlude();
        //     match self.position {
        //         TabBarPosition::Right => deferred(
        //             handle
        //                 .absolute()
        //                 .top(px(0.))
        //                 .left(-RESIZE_HANDLE_SIZE / 2.)
        //                 .h_full()
        //                 .w(RESIZE_HANDLE_SIZE)
        //                 .cursor_col_resize(),
        //         ),
        //         _ => {}
        //     }
        // };

        tab_bar
            .flex()
            .id(self.id)
            .group("tab_bar")
            .h(
                // TODO: This should scale with [UiDensity], however tabs,
                // and other tab bar tools need to scale dynamically first.
                rems_from_px(29.),
            )
            .bg(cx.theme().colors().tab_bar_background)
            // TODO change this condition
            // .when(!self.start_children.is_empty(), |this| {
            //     let mut child = h_flex()
            //         .flex_none()
            //         .gap(Spacing::Small.rems(cx))
            //         .px(Spacing::Medium.rems(cx))
            //         .border_b_1()
            //         .border_r_1()
            //         .border_color(cx.theme().colors().border)
            //         .children(self.start_children);
            //     // if self.position == TabBarPosition::Right && !self.end_children.is_empty() {
            //     //     child = child.children(self.end_children);
            //     // }
            //     this.child(child)
            // })
            .child(
                tab_buttons_container
                    // .relative()
                    // .flex_1()
                    // .overflow_x_hidden()
                    .child(
                        div()
                            .absolute()
                            .top_0()
                            .left_0()
                            .size_full()
                            .border_b_1()
                            .border_color(cx.theme().colors().border),
                    )
                    .child(
                        tabs.id("tabs")
                            .overflow_x_scroll()
                            .when_some(self.scroll_handle, |cx, scroll_handle| {
                                cx.track_scroll(&scroll_handle)
                            })
                            .child(v_tab_bar_buttons)
                            .children(self.children),
                    ),
            )
        // .child(create_resize_handle())
        // .when(
        //     !self.end_children.is_empty(), /*&& self.position != TabBarPosition::Right*/
        //     |this| {
        //         this.child(
        //             h_flex()
        //                 .flex_none()
        //                 .gap(Spacing::Small.rems(cx))
        //                 .px(Spacing::Medium.rems(cx))
        //                 .border_b_1()
        //                 .border_l_1()
        //                 .border_color(cx.theme().colors().border)
        //                 .children(self.end_children),
        //         )
        //     },
        // )
    }
}
