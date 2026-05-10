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
use yew::{function_component, html, Html};

#[function_component(StyleGuide)]
pub fn dashboard_page() -> Html {
    html! {
        <div class="style-guide">
            <h1>{ "KelpieBooks UI System - Style guide" }</h1>

            // --- COLORS SECTION ---
            <section class="style-guide__section">
                <h2>{ "Brand Colors" }</h2>
                <div class="style-guide__row">
                    <div class="style-guide__item">
                        <div style="width: 100px; height: 100px; border-radius: 8px; background: #65ddcd;"></div>
                        <span>{ "Primary ($brand-primary)" }</span>
                    </div>
                    <div class="style-guide__item">
                        <div style="width: 100px; height: 100px; border-radius: 8px; background: #3b0000;"></div>
                        <span>{ "Dark ($brand-dark)" }</span>
                    </div>
                </div>
            </section>

            // --- TYPOGRAPHY SECTION ---
            <section class="style-guide__section">
                <h2>{ "Typography" }</h2>
                <h1>{ "Heading 1 - Accounting Title" }</h1>
                <h2>{ "Heading 2 - Section Header" }</h2>
                <p>{ "Standard paragraph text for reports and descriptions." }</p>
                <p class="text-amount">{ "1,234,567.89 (Tabular Numbers)" }</p>
            </section>

            // --- BUTTONS SECTION ---
            <section class="style-guide__section">
                <h2>{ "Buttons" }</h2>
                <div class="style-guide__row">
                    <div class="style-guide__item">
                        <button class="btn-primary">{ "Save Changes" }</button>
                        <span>{ ".btn-primary" }</span>
                    </div>
                    <div class="style-guide__item">
                        <button class="btn-secondary">{ "Cancel" }</button>
                        <span>{ ".btn-secondary" }</span>
                    </div>
                    <div class="style-guide__item">
                        <button disabled=true>{ "Disabled" }</button>
                        <span>{ ":disabled" }</span>
                    </div>
                </div>
            </section>

            // --- FORM ELEMENTS ---
            <section class="style-guide__section">
                <h2>{ "Inputs" }</h2>
                <div class="style-guide__row">
                    <div class="style-guide__item">
                        <label>{ "Account Name" }</label>
                        <input type="text" placeholder="e.g. Accounts Receivable" />
                    </div>
                    <div class="style-guide__item">
                        <label>{ "Status" }</label>
                        <select>
                            <option>{ "Active" }</option>
                            <option>{ "Archived" }</option>
                        </select>
                    </div>
                </div>
            </section>
            <section class="style-guide__section">
                <div class="report-header"><h2>{ "Financial Reports" }</h2>
                    <div class="report__action-bar">
                        <div class="report__date-range-selector">
                            <label>{ "From: " }</label>
                            <input type="date" />
                            <label>{ "To: " }</label>
                            <input type="date" />
                            <button class="icon-button" title="Export to CSV">
                                <img src="/images/download.svg" alt="Export CSV" />
                            </button>
                            <button class="icon-button" title="Export to PDF">
                                <img src="/images/export-pdf.svg" alt="Export PDF" />
                            </button>
                        </div>
                    </div>
                </div>
                <table class="report-table">
                    <tr class="report__section-header">
                        <td colspan="2">{ "Assets" }</td>
                    </tr>
                    <tr>
                        <td>{ "Petty Cash" }</td>
                        <td class="text-amount">{ "$100.00" }</td>
                    </tr>
                    <tr class="report__total-row">
                        <td>{ "Total Assets" }</td>
                        <td class="text-amount">{ "$100.00" }</td>
                    </tr>
                </table>
            </section>
        </div>
    }
}