/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use yew::prelude::*;
use crate::api::Api;
use crate::contexts::auth_context::use_user_context;
use crate::contexts::locale_context::use_locale;
use fluent::fluent_args;
use shared_core::sales::models::tax::TaxCategory;
use yew_router::prelude::use_navigator;
use shared_core::core::models::auth::SystemPrivilege;
use crate::sales::components::tax_category_row::TaxCategoryRow;
use crate::sales::components::add_tax_category_modal::AddTaxCategoryModal;
use crate::core::components::delete_confirmation_modal::DeleteConfirmationModal;
use crate::sales::components::tax_category_drawer::tax_category_drawer::Tab;

#[derive(Properties, PartialEq, Clone)]
pub struct TaxCategoryListTableProps {
    pub on_category_select: Callback<(TaxCategory, Tab)>,
}

#[function_component(TaxCategoryListTable)]
pub fn tax_category_list_table(props: &TaxCategoryListTableProps) -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let tax_categories = use_state(Vec::new);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);
    let show_add_modal = use_state(|| false);
    let tax_category_to_delete = use_state(|| None::<TaxCategory>);

    let fetch_tax_categories = {
        let tax_categories = tax_categories.clone();
        let error = error.clone();
        let loading = loading.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        Callback::from(move |_: ()| {
            let tax_categories = tax_categories.clone();
            let error = error.clone();
            let loading = loading.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            loading.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_tax_categories = Api::get("/api/sales/tax-categories", user_ctx, navigator).await;
                loading.set(false);
                match fetched_tax_categories {
                    Ok(response) if response.ok() => {
                        match response.json::<Vec<TaxCategory>>().await {
                            Ok(data) => tax_categories.set(data),
                            Err(e) => error.set(Some(i18n.t_args(
                                "tax-category-list-error-parse-tax-categories",
                                &fluent_args!["error" => e.to_string()],
                            ))),
                        }
                    }
                    Ok(response) => error.set(Some(i18n.t_args(
                        "tax-category-list-error-fetch-tax-categories",
                        &fluent_args!["status" => response.status()],
                    ))),
                    Err(e) => error.set(Some(i18n.t_args(
                        "common-network-error",
                        &fluent_args!["error" => e.to_string()],
                    ))),
                }
            });
        })
    };

    let fetch_tax_categories_clone = fetch_tax_categories.clone();
    use_effect_with((), move |()| {
        fetch_tax_categories_clone.emit(());
        || ()
    });

    let on_add_click = {
        let show_add_modal = show_add_modal.clone();
        Callback::from(move |_| show_add_modal.set(true))
    };

    let on_delete_click = {
        let tax_category_to_delete = tax_category_to_delete.clone();
        Callback::from(move |tax_category: TaxCategory| {
            tax_category_to_delete.set(Some(tax_category));
        })
    };

    let on_modal_close = {
        let show_add_modal = show_add_modal.clone();
        let tax_category_to_delete = tax_category_to_delete.clone();
        Callback::from(move |_: ()| {
            show_add_modal.set(false);
            tax_category_to_delete.set(None);
        })
    };

    let on_submit = {
        let fetch_tax_categories = fetch_tax_categories.clone();
        let on_modal_close = on_modal_close.clone();
        Callback::from(move |_: ()| {
            fetch_tax_categories.emit(());
            on_modal_close.emit(());
        })
    };

    let on_delete_confirm = {
        let on_submit = on_submit.clone();
        let tax_category_to_delete = tax_category_to_delete.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let error = error.clone();
        let i18n = i18n.clone();

        Callback::from(move |_| {
            let on_submit = on_submit.clone();
            let tax_category_to_delete = tax_category_to_delete.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            let error = error.clone();
            let i18n = i18n.clone();

            if let Some(tax_category) = &*tax_category_to_delete {
                let tax_category_id = tax_category.id;
                wasm_bindgen_futures::spawn_local(async move {
                    let resp = Api::delete(&format!("/api/sales/tax-categories/{}", tax_category_id), user_ctx, navigator).await;
                    if resp.is_ok() {
                        on_submit.emit(());
                    } else {
                        error.set(Some(i18n.t("tax-category-delete-error")));
                    }
                });
            }
        })
    };

    if *loading {
        return html! { <p>{ i18n.t("common-loading") }</p> };
    }
    if let Some(err) = &*error {
        return html! { <div class="error">{ err }</div> };
    }

    html! {
        <>
            <div class="table-actions">
                { if user_ctx.has_privilege(&SystemPrivilege::ManageSales) {
                    html! {
                        <button onclick={on_add_click}>{ i18n.t("tax-category-list-add-tax-category-button") }</button>
                    }
                } else {
                    html! {}
                }}
            </div>

            if *show_add_modal {
                <AddTaxCategoryModal on_close={on_modal_close.clone()} on_submit={on_submit.clone()} />
            }
            if let Some(tax_category) = &*tax_category_to_delete {
                <DeleteConfirmationModal
                    title={i18n.t("tax-category-delete-title")}
                    message={i18n.t_args("tax-category-delete-confirm-message", &fluent_args!["name" => tax_category.name.clone()])}
                    on_confirm={on_delete_confirm}
                    on_cancel={on_modal_close}
                />
            }

            <table class="table">
                <thead>
                    <tr>
                        <th class="table__text-col">{ i18n.t("tax-category-list-name") }</th>
                        <th class="table__text-col">{ i18n.t("common-description") }</th>
                        <th class="table__text-col">{ i18n.t("tax-category-list-is-active") }</th>
                        <th class="table__col-actions">{ i18n.t("common-actions") }</th>
                    </tr>
                </thead>
                <tbody>
                    { for (*tax_categories).iter().map(|tax_category| {
                        let on_view = {
                            let on_category_select = props.on_category_select.clone();
                            let tax_category = tax_category.clone();
                            Callback::from(move |_| {
                                on_category_select.emit((tax_category.clone(), Tab::General));
                            })
                        };
                        let on_select = {
                            let on_category_select = props.on_category_select.clone();
                            let tax_category = tax_category.clone();
                            Callback::from(move |_| {
                                on_category_select.emit((tax_category.clone(), Tab::Rates));
                            })
                        };
                        html! {
                            <TaxCategoryRow
                                tax_category={tax_category.clone()}
                                on_view={on_view}
                                on_delete={on_delete_click.clone()}
                                on_select={on_select}
                            />
                        }
                    })}
                </tbody>
            </table>
        </>
    }
}