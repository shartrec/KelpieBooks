/*
 * Copyright (c) 2026-2026. Trevor Campbell and others.
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

use yew::prelude::*;
use chrono::{NaiveDate, Datelike};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DateRange {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

impl Default for DateRange {
    fn default() -> Self {
        let now = chrono::Local::now().date_naive();
        let start = NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap_or(now);
        Self {
            start_date: start,
            end_date: now,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ReportAction {
    SetDateRange(DateRange),
    SetOnExportCsv(Option<Callback<()>>),
    SetOnExportTypst(Option<Callback<()>>),
}

pub type ReportState = ReportContextData;

#[derive(Clone, Debug, PartialEq)]
pub struct ReportContextData {
    pub date_range: DateRange,
    pub on_export_csv: Option<Callback<()>>,
    pub on_export_typst: Option<Callback<()>>,
}

impl Default for ReportContextData {
    fn default() -> Self {
        let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        let date_range = storage.get_item("report_date_range")
            .unwrap_or_default()
            .and_then(|s| serde_json::from_str::<DateRange>(&s).ok())
            .unwrap_or_default();
        Self {
            date_range,
            on_export_csv: None,
            on_export_typst: None,
        }
    }
}

impl Reducible for ReportContextData {
    type Action = ReportAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            ReportAction::SetDateRange(date_range) => {
                let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
                let _ = storage.set_item("report_date_range", &serde_json::to_string(&date_range).unwrap());
                Self {
                    date_range,
                    ..(*self).clone()
                }.into()
            }
            ReportAction::SetOnExportCsv(on_export_csv) => {
                if self.on_export_csv == on_export_csv {
                    self
                } else {
                    Self {
                        on_export_csv,
                        ..(*self).clone()
                    }.into()
                }
            }
            ReportAction::SetOnExportTypst(on_export_typst) => {
                if self.on_export_typst == on_export_typst {
                    self
                } else {
                    Self {
                        on_export_typst,
                        ..(*self).clone()
                    }.into()
                }
            }
        }
    }
}

pub type ReportContext = UseReducerHandle<ReportContextData>;

#[derive(Properties, PartialEq)]
pub struct ReportContextProviderProps {
    pub children: Children,
}

#[function_component(ReportContextProvider)]
pub fn report_context_provider(props: &ReportContextProviderProps) -> Html {
    let context = use_reducer(ReportContextData::default);

    html! {
        <ContextProvider<ReportContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<ReportContext>>
    }
}

#[hook]
pub fn use_report_context() -> ReportContext {
    use_context::<ReportContext>().expect("ReportContext not found")
}
