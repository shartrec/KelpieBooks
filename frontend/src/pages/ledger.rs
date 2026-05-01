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

use crate::components::chart_of_accounts_table::ChartOfAccountsTable;
use crate::components::layout::Layout;
use crate::contexts::report_context::{use_report_context, ReportAction};
use yew::prelude::*;

#[function_component(LedgerPage)]
pub fn ledger_page() -> Html {
    let report_ctx = use_report_context();

    use_effect_with((), move |_| {
        let on_export_csv = Callback::from(|_| {
            web_sys::window().unwrap().alert_with_message("Exporting Chart of Accounts to CSV...").unwrap();
        });
        let on_export_typst = Callback::from(|_| {
            web_sys::window().unwrap().alert_with_message("Exporting Chart of Accounts to Typst...").unwrap();
        });
        report_ctx.dispatch(ReportAction::SetOnExportCsv(Some(on_export_csv)));
        report_ctx.dispatch(ReportAction::SetOnExportTypst(Some(on_export_typst)));
        move || {
            report_ctx.dispatch(ReportAction::SetOnExportCsv(None));
            report_ctx.dispatch(ReportAction::SetOnExportTypst(None));
        }
    });

    html! {
        <Layout>
            <h1>{ "Chart of Accounts" }</h1>
            <p>{ "This is a list of all accounts in your organization. The balances include all transactions and are rolled up into parent accounts." }</p>
            <ChartOfAccountsTable />
        </Layout>
    }
}
