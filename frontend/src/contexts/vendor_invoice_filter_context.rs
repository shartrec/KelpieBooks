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

use chrono::{Datelike, NaiveDate};
use std::rc::Rc;
use uuid::Uuid;
use yew::html::ChildrenProps;
use yew::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentStatusFilter {
    All,
    Outstanding,
    Paid,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VendorInvoiceFilterState {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub partner_id: Option<Uuid>,
    pub min_amount: Option<i64>,
    pub status: PaymentStatusFilter,
}

impl Default for VendorInvoiceFilterState {
    fn default() -> Self {
        let today = chrono::Local::now().date_naive();
        Self {
            start_date: today.with_day(1).unwrap(),
            end_date: today,
            partner_id: None,
            min_amount: None,
            status: PaymentStatusFilter::Outstanding,
        }
    }
}

pub enum VendorInvoiceFilterAction {
    SetStartDate(NaiveDate),
    SetEndDate(NaiveDate),
    SetPartnerId(Option<Uuid>),
    SetMinAmount(Option<i64>),
    SetStatus(PaymentStatusFilter),
}

impl Reducible for VendorInvoiceFilterState {
    type Action = VendorInvoiceFilterAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut next_state = (*self).clone();
        match action {
            VendorInvoiceFilterAction::SetStartDate(date) => next_state.start_date = date,
            VendorInvoiceFilterAction::SetEndDate(date) => next_state.end_date = date,
            VendorInvoiceFilterAction::SetPartnerId(id) => next_state.partner_id = id,
            VendorInvoiceFilterAction::SetMinAmount(amount) => next_state.min_amount = amount,
            VendorInvoiceFilterAction::SetStatus(status) => next_state.status = status,
        }
        next_state.into()
    }
}

pub type VendorInvoiceFilterContext = UseReducerHandle<VendorInvoiceFilterState>;

#[function_component(VendorInvoiceFilterProvider)]
pub fn vendor_invoice_filter_provider(props: &ChildrenProps) -> Html {
    let filter_state = use_reducer(VendorInvoiceFilterState::default);

    html! {
        <ContextProvider<VendorInvoiceFilterContext> context={filter_state}>
            {props.children.clone()}
        </ContextProvider<VendorInvoiceFilterContext>>
    }
}

#[hook]
pub fn use_vendor_invoice_filter() -> VendorInvoiceFilterContext {
    use_context::<VendorInvoiceFilterContext>().expect("No VendorInvoiceFilterContext found")
}
