/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use shared_core::util::info;
use yew::prelude::*;

use crate::contexts::locale_context::use_locale;

#[derive(Properties, PartialEq)]
pub struct AboutModalProps {
    pub on_close: Callback<()>,
}

#[function_component(AboutModal)]
pub fn about_modal(props: &AboutModalProps) -> Html {
    let i18n = use_locale();

    let on_close = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };

    html! {
        <div class="drawer-overlay" onclick={on_close.clone()}>
            <div class="modal about-modal" onclick={|e: MouseEvent| e.stop_propagation()}>
                <header class="drawer__header">
                    <h3>{ i18n.t("about-title") }</h3>
                    <button class="btn-close" type="button" onclick={on_close.clone()}>
                        <img src="/images/x.svg" alt={i18n.t("common-close")} />
                    </button>
                </header>

                <div class="modal__body about-modal__body">
                    <div class="about-modal__logo-row">
                        <img
                            src="/images/kelpiedog_120x120_transparent.png"
                            alt={i18n.t("sidebar-logo-alt")}
                            class="about-modal__logo"
                        />
                        <div class="about-modal__title-block">
                            <h2 class="about-modal__app-name">{ info::PROGRAM_NAME }</h2>
                            <span class="about-modal__version">
                                { i18n.t("about-version") }{ " " }{ info::VERSION }
                            </span>
                        </div>
                    </div>

                    <p class="about-modal__description">{ i18n.t("about-description") }</p>

                    <dl class="about-modal__details">
                        <dt>{ i18n.t("about-author-label") }</dt>
                        <dd>{ info::AUTHOR }</dd>

                        <dt>{ i18n.t("about-license-label") }</dt>
                        <dd>{ info::LICENSE_TYPE }</dd>

                        <dt>{ i18n.t("about-website-label") }</dt>
                        <dd>
                            <a href={info::WEBSITE} target="_blank" rel="noopener noreferrer">
                                { info::WEBSITE }
                            </a>
                        </dd>

                        <dt>{ i18n.t("about-docs-label") }</dt>
                        <dd>
                            <a href={info::DOCSITE} target="_blank" rel="noopener noreferrer">
                                { info::DOCSITE }
                            </a>
                        </dd>
                    </dl>
                </div>

                <footer class="drawer__footer">
                    <button class="button-secondary" onclick={on_close}>
                        { i18n.t("common-close") }
                    </button>
                </footer>
            </div>
        </div>
    }
}
