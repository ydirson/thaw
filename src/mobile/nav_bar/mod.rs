use crate::{components::*, icon::*, utils::mount_style::mount_style};
use leptos::*;
use web_sys::MouseEvent;

#[component]
pub fn NavBar(
    #[prop(optional, into)] title: MaybeSignal<&'static str>,
    #[prop(optional, into)] left_arrow: MaybeSignal<bool>,
    #[prop(optional, into)] left_text: MaybeSignal<&'static str>,
    #[prop(optional, into)] click_left: Option<SignalSetter<MouseEvent>>,
    #[prop(optional, into)] right_text: MaybeSignal<&'static str>,
    #[prop(optional, into)] click_right: Option<SignalSetter<MouseEvent>>,
) -> impl IntoView {
    mount_style("nav-bar", include_str!("./nav-bar.css"));

    let onclick_left = move |ev| {
        if let Some(click_left) = click_left {
            click_left.set(ev);
        }
    };
    let onclick_right = move |ev| {
        if let Some(click_right) = click_right {
            click_right.set(ev);
        }
    };

    view! {
        <div class="melt-nav-bar">
            <If cond=MaybeSignal::derive( move || left_arrow.get() || !left_text.get().is_empty())>
                <Then slot>
                    <div class="melt-nav-bar__left" on:click=onclick_left>
                        <If cond=left_arrow>
                            <Then slot>
                                <Icon icon=Icon::from(AiIcon::AiLeftOutlined)/>
                            </Then>
                        </If>
                        { left_text.get() }
                    </div>
                </Then>
            </If>
            <div class="melt-nav-bar__center">
                { move || title.get() }
            </div>
            <If cond=MaybeSignal::derive( move || !right_text.get().is_empty())>
                <Then slot>
                    <div class="melt-nav-bar__right" on:click=onclick_right>
                        { right_text.get() }
                    </div>
                </Then>
            </If>
        </div>
    }
}
