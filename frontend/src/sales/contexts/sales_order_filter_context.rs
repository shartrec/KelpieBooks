/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use std::rc::Rc;

use chrono::{
    Datelike,
    NaiveDate,
};
use rust_decimal::Decimal;
use shared_core::PartnerId;
use yew::{
    html::ChildrenProps,
    prelude::*,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentStatusFilter {
    Draft,
    All,
    Outstanding,
    Paid,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SalesOrderFilterState {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub partner_id: Option<PartnerId>,
    pub min_amount: Option<Decimal>,
    pub status: PaymentStatusFilter,
}

impl Default for SalesOrderFilterState {
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

pub enum SalesOrderFilterAction {
    SetStartDate(NaiveDate),
    SetEndDate(NaiveDate),
    SetPartnerId(Option<PartnerId>),
    SetMinAmount(Option<Decimal>),
    SetStatus(PaymentStatusFilter),
}

impl Reducible for SalesOrderFilterState {
    type Action = SalesOrderFilterAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut next_state = (*self).clone();
        match action {
            SalesOrderFilterAction::SetStartDate(date) => next_state.start_date = date,
            SalesOrderFilterAction::SetEndDate(date) => next_state.end_date = date,
            SalesOrderFilterAction::SetPartnerId(id) => next_state.partner_id = id,
            SalesOrderFilterAction::SetMinAmount(amount) => next_state.min_amount = amount,
            SalesOrderFilterAction::SetStatus(status) => next_state.status = status,
        }
        next_state.into()
    }
}

pub type SalesOrderFilterContext = UseReducerHandle<SalesOrderFilterState>;

#[function_component(SalesOrderFilterProvider)]
pub fn sales_invoice_filter_provider(props: &ChildrenProps) -> Html {
    let filter_state = use_reducer(SalesOrderFilterState::default);

    html! {
        <ContextProvider<SalesOrderFilterContext> context={filter_state}>
            {props.children.clone()}
        </ContextProvider<SalesOrderFilterContext>>
    }
}

#[hook]
pub fn use_sales_order_filter() -> SalesOrderFilterContext {
    use_context::<SalesOrderFilterContext>().expect("No SalesOrderFilterContext found")
}
