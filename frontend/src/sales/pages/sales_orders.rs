/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use yew::prelude::*;

use crate::{
    core::components::layout::Layout,
};
use crate::sales::components::sales_order_list::SalesOrdersList;
use crate::sales::contexts::sales_order_filter_context::SalesOrderFilterProvider;

#[function_component(SalesOrdersPage)]
pub fn sales_orders_page() -> Html {
    html! {
        <Layout>
            <SalesOrderFilterProvider>
                <SalesOrdersList />
            </SalesOrderFilterProvider>
        </Layout>
    }
}
