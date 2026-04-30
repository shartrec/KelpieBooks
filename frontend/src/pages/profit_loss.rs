/*
 * Copyright (c) 2026. Trevor Campbell and others.
 *
 * This file is part of KelpieBooks.
 *
 * KelpieBooks is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License,or
 * (at your option) any later version.
 *
 * KelpieBooks is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with KelpieBooks; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 *
 * Contributors:
 *      Trevor Campbell
 *
 */

use crate::components::layout::Layout;
use crate::contexts::report_context::{use_report_context, ReportAction};
use yew::prelude::*;

#[function_component(ProfitLossPage)]
pub fn profit_loss_page() -> Html {
    let report_ctx = use_report_context();

    use_effect_with((), move |_| {
        let on_export = Callback::from(|_| {
            web_sys::window().unwrap().alert_with_message("Exporting Profit & Loss...").unwrap();
        });
        report_ctx.dispatch(ReportAction::SetOnExport(Some(on_export)));
        move || report_ctx.dispatch(ReportAction::SetOnExport(None))
    });

    html! {
        <Layout>
            <h1>{ "Profit & Loss" }</h1>
            <p>{ "This is a placeholder for the Profit & Loss report." }</p>
        </Layout>
    }
}
