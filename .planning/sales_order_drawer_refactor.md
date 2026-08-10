# Sales Order Drawer Refactor — Plan

## Overview

Refactor the single-file `sales_order_drawer.rs` component into a sub-directory module following
the same structure as `sales_invoice_drawer/`. Each tab becomes its own focused Yew component,
and the items view is upgraded from a plain table to the card-based layout used in
`sales_invoice_drawer/items_view.rs`.

### Goals
- Mirror the `sales_invoice_drawer/` directory structure exactly.
- Replace the raw `<table>` items view with `card-item-compact` cards.
- Make each view independently extensible (e.g. adding edit/delete to items later).
- The root `SalesOrderDrawer` component's public API (props) does **not** change — callers in
  `sales_orders.rs` are unaffected.

### Non-Goals
- Adding edit/delete actions to order lines (read-only for now; structure just allows it later).
- Changing the confirm/cancel business logic.
- Any backend changes.

---

## Sub-Tasks

---

### Sub-Task 1 — Scaffold the sub-directory module

**Status:** [ ] pending

**Intent**
Convert `sales_order_drawer.rs` (single file) into a `sales_order_drawer/` directory module,
keeping the existing behaviour intact while establishing the file structure.

**Expected Outcomes**
- `frontend/src/sales/components/sales_order_drawer/mod.rs` — contains the root
  `SalesOrderDrawer` function component (same logic as the current single file).
- `frontend/src/sales/components/sales_order_drawer/lines_view.rs` — stub file that re-exports
  the `render_lines_tab` logic as a `LinesView` function component (props: `order: SalesOrder`).
- `frontend/src/sales/components/sales_order_drawer/addresses_view.rs` — stub file that
  re-exports the `render_addresses_tab` logic as an `AddressesView` function component
  (props: `order: SalesOrder`).
- `frontend/src/sales/components/mod.rs` reference updated from
  `pub mod sales_order_drawer;` (file) to `pub mod sales_order_drawer;` (directory) — no
  change needed if Rust resolves the directory automatically, but verify.
- The old `sales_order_drawer.rs` file is deleted.
- `cargo check --workspace` still passes.

**Todo List**
1. Create directory `frontend/src/sales/components/sales_order_drawer/`.
2. Create `mod.rs` — copy the current `sales_order_drawer.rs` content, replacing the two
   free-function render calls with `<LinesView order={...} />` and `<AddressesView order={...} />`.
3. Create `lines_view.rs` — move the `render_lines_tab` free function into a
   `#[function_component(LinesView)]` with `LinesViewProps { order: SalesOrder }`.
4. Create `addresses_view.rs` — move `render_addresses_tab` into
   `#[function_component(AddressesView)]` with `AddressesViewProps { order: SalesOrder }`.
5. Delete the old `frontend/src/sales/components/sales_order_drawer.rs` file.
6. Run `cargo check --workspace`; fix any errors.

**Relevant Context**
- `frontend/src/sales/components/sales_invoice_drawer/mod.rs` is the pattern for the root
  component — tabs delegate to child components via `html! { <ChildComponent ... /> }`.
- `frontend/src/sales/components/sales_invoice_drawer/addresses_view.rs` is the model for
  `AddressesView`.
- The `mod.rs` import of child components follows: `use crate::sales::components::sales_order_drawer::{lines_view::LinesView, addresses_view::AddressesView};`

---

### Sub-Task 2 — Upgrade `LinesView` to card layout

**Status:** [ ] pending

**Intent**
Replace the plain `<table>` in `lines_view.rs` with the `card-item-compact` card layout used
in `sales_invoice_drawer/items_view.rs`, so each order line is displayed as a compact card
showing item name/quantity on one line and financial breakdown on the other. Retain the
availability badge as a prominent element on each card.

**Expected Outcomes**
- Each order line renders as a `<div class="card-item-compact">` with:
  - **Meta row** (`card-item-compact__meta`): item code + name + quantity badge on the left;
    availability badge (`status-badge--confirmed` / `status-badge--warning` / neutral) on the
    right.
  - **Body row** (`card-item-compact__body`): description on the left; financial breakdown
    (gross total large, "net + tax" sub-line) on the right, matching the `card-item-compact__financials`
    pattern from `items_view.rs`.
  - The running total footer (`voucher-footer`) remains at the bottom of the view.
- The plain `<table>` and `<thead>`/`<tbody>` are gone.
- `cargo check --workspace` passes.

**Todo List**
1. Rewrite `lines_view.rs` — replace the `<table>` html block with card markup.
2. Meta row: `{ format!("{} × {}", line.quantity_display, line.name) }` on the left (use
   `i18n.format_decimal(line.quantity)` for the quantity); availability badge on the right.
3. Body row: `line.description` as `card-item-compact__desc`; gross + net/tax breakdown as
   `card-item-compact__financials` / `card-item-compact__total` / `card-item-compact__sub-breakdown`.
4. Use the `items-view-net-tax-breakdown` i18n key (already present in en.ftl) for the sub-line.
5. Retain the `voucher-footer` with the order total at the bottom.
6. Run `cargo check --workspace`; fix any errors.

**Relevant Context**
- `sales_invoice_drawer/items_view.rs` lines 165–210 show the exact `card-item-compact` HTML
  structure to follow.
- The availability badge logic from the original `render_lines_tab` is preserved; just moved
  to the right side of the meta row instead of a table column.
- `items-view-net-tax-breakdown` translation key is already in `en.ftl` and `fr.ftl`.
- No edit/delete buttons on order lines yet — the `card__actions` div is omitted for now
  (add later when order line editing is implemented).

---

### Sub-Task 3 — Upgrade `AddressesView` to full card component

**Status:** [ ] pending

**Intent**
Upgrade the addresses view from a simple read-only card grid to match the richer card style
in `sales_invoice_drawer/addresses_view.rs` — proper `card__header`, `card__body`,
`card__footer` structure with the edit button placeholder in the footer (disabled for now,
since order addresses are currently immutable after creation).

**Expected Outcomes**
- Each address renders as a full `card` with `card__header` (address type label),
  `card__body` (address lines), and `card__footer` (edit button, disabled, for future use).
- Address type label uses `address_type.to_string()` (already implemented via the `Display`
  trait on `AddressType`).
- The edit button is present but `disabled=true` with a tooltip i18n key
  `sales-order-drawer-address-edit-future` ("Address editing coming soon").
- Translation keys added to `en.ftl` and `fr.ftl`.
- `cargo check --workspace` passes.

**Todo List**
1. Rewrite `addresses_view.rs` to use the full `card` / `card__header` / `card__body` /
   `card__footer` structure.
2. Add disabled edit button in `card__footer`.
3. Add `sales-order-drawer-address-edit-future` key to `en.ftl` and `fr.ftl`.
4. Run `cargo check --workspace`; fix any errors.

**Relevant Context**
- `sales_invoice_drawer/addresses_view.rs` is the model — copy the card structure (without the
  editing state machine, since orders don't support address editing yet).
- The `AddressType` enum's `to_string()` is used directly as the card header title.
- CSS classes `card`, `card--primary-billing`, `card--primary-shipping` are already in
  `kelpie.css`.

