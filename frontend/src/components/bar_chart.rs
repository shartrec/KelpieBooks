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

use shared_core::dtos::expense_breakdown::ExpenseBreakdown;
use shared_core::util::format_currency;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BarChartProps {
    pub data: Vec<ExpenseBreakdown>,
}

#[function_component(BarChart)]
pub fn bar_chart(props: &BarChartProps) -> Html {
    let max_amount = props.data.iter().map(|d| d.amount).fold(0, i64::max);

    html! {
        <div class="space-y-2">
            { for props.data.iter().map(|d| {
                let width = if max_amount > 0 {
                    (d.amount / max_amount * 100) as u32
                } else {
                    0
                };
                html! {
                    <div class="flex items-center">
                        <div class="w-1/4 text-sm">{ &d.category }</div>
                        <div class="w-3/4">
                            <div class="bg-primary h-6 rounded-md" style={format!("width: {}%", width)}></div>
                        </div>
                        <div class="w-1/4 text-sm text-right">{ format_currency(&d.amount) }</div>
                    </div>
                }
            })}
        </div>
    }
}
