/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use std::rc::Rc;

use chrono::{Datelike, NaiveDate};
use rust_decimal::Decimal;
use uuid::Uuid;
use yew::{html::ChildrenProps, prelude::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentStatusFilter {
    All,
    Outstanding,
    Paid,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SalesInvoiceFilterState {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub partner_id: Option<Uuid>,
    pub min_amount: Option<Decimal>,
    pub status: PaymentStatusFilter,
}

impl Default for SalesInvoiceFilterState {
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

pub enum SalesInvoiceFilterAction {
    SetStartDate(NaiveDate),
    SetEndDate(NaiveDate),
    SetPartnerId(Option<Uuid>),
    SetMinAmount(Option<Decimal>),
    SetStatus(PaymentStatusFilter),
}

impl Reducible for SalesInvoiceFilterState {
    type Action = SalesInvoiceFilterAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut next_state = (*self).clone();
        match action {
            SalesInvoiceFilterAction::SetStartDate(date) => next_state.start_date = date,
            SalesInvoiceFilterAction::SetEndDate(date) => next_state.end_date = date,
            SalesInvoiceFilterAction::SetPartnerId(id) => next_state.partner_id = id,
            SalesInvoiceFilterAction::SetMinAmount(amount) => next_state.min_amount = amount,
            SalesInvoiceFilterAction::SetStatus(status) => next_state.status = status,
        }
        next_state.into()
    }
}

pub type SalesInvoiceFilterContext = UseReducerHandle<SalesInvoiceFilterState>;

#[function_component(SalesInvoiceFilterProvider)]
pub fn sales_invoice_filter_provider(props: &ChildrenProps) -> Html {
    let filter_state = use_reducer(SalesInvoiceFilterState::default);

    html! {
        <ContextProvider<SalesInvoiceFilterContext> context={filter_state}>
            {props.children.clone()}
        </ContextProvider<SalesInvoiceFilterContext>>
    }
}

#[hook]
pub fn use_sales_invoice_filter() -> SalesInvoiceFilterContext {
    use_context::<SalesInvoiceFilterContext>().expect("No SalesInvoiceFilterContext found")
}
