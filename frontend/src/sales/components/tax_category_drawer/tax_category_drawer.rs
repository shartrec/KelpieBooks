/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use yew::prelude::*;
use shared_core::sales::models::tax::TaxCategory;
use crate::contexts::locale_context::use_locale;
use crate::sales::components::tax_category_drawer::{
    general_view::GeneralView,
    rates_view::RatesView,
};

#[derive(Properties, PartialEq, Clone)]
pub struct TaxCategoryDrawerProps {
    pub category: TaxCategory,
    pub on_close: Callback<()>,
    pub on_change: Callback<()>,
    pub initial_tab: Tab,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Tab {
    General,
    Rates,
}

#[function_component(TaxCategoryDrawer)]
pub fn tax_category_drawer(props: &TaxCategoryDrawerProps) -> Html {
    let i18n = use_locale();
    let active_tab = use_state(|| props.initial_tab);
    let error = use_state(|| None::<String>);

    let set_tab = |tab: Tab| {
        let active_tab = active_tab.clone();
        Callback::from(move |_| {
            active_tab.set(tab);
        })
    };
    let on_close = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| {
            on_close.emit(());
        })
    };

    html! {
        <div class="drawer-overlay" onclick={on_close.clone()}>
            <div class="drawer" onclick={|e: MouseEvent| e.stop_propagation()}>
                <header class="drawer__header">
                    <h3>{ &props.category.name } </h3>
                        <button class="btn-close" type="button" onclick={on_close.clone()}>
                            <img src="/images/x.svg" alt={i18n.t("common-close")} />
                        </button>
                </header>
                <div class="drawer__tabs">
                    <button
                        class={classes!("tab-trigger", (*active_tab == Tab::General).then_some("tab-trigger--active"))}
                        onclick={set_tab(Tab::General)}
                    >
                        { i18n.t("common-general") }
                    </button>
                    <button
                        class={classes!("tab-trigger", (*active_tab == Tab::Rates).then_some("tab-trigger--active"))}
                        onclick={set_tab(Tab::Rates)}
                    >
                        { i18n.t("tax-category-row-manage-rates") }
                    </button>
                </div>
                <div class="drawer__content">
                    if let Some(e) = &*error {
                        <div class="error">{e}</div>
                    }
                    {
                        match *active_tab {
                            Tab::General => html! { <GeneralView tax_category={props.category.clone()} on_change={props.on_change.clone()} /> },
                            Tab::Rates => html! { <RatesView category={props.category.clone()} on_change={props.on_change.clone()} /> },
                        }
                    }
                </div>
                <footer class="drawer__footer">
                    <button class="button-secondary" onclick={on_close.clone()}>{ i18n.t("common-close") }</button>
                </footer>
            </div>
        </div>
    }
}