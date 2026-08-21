# Sales Order Entry — Plan

## Overview

Add a **Sales Order** workflow that allows a user to take a customer order (e.g. over the telephone) before
invoicing. A Sales Order is a confirmed customer commitment that:

- Captures customer, lines, quantities, prices, and addresses — identical to `new_sales_invoice.rs` in structure.
- Requires a **warehouse** to be selected on the order (used for stock availability display and allocation).
- Allocates stock against `stocked` items in the selected warehouse immediately on confirmation.
- Converts directly to a **Sales Invoice** (status `Open`) on the "Confirm Order" action.
- Lays the groundwork for a future pick/ship/invoice flow (the stock allocation step is already in place).

### Design Decisions (confirmed)

1. **Confirmed orders are hidden from the default list view.** The orders list defaults to `status = Open`
   only. Confirmed and cancelled orders are accessible via a status filter.
2. **A warehouse must be selected when creating the order.** Stock availability is shown per warehouse.
   Allocation on confirmation targets the chosen warehouse (picking location selection is a future concern).
3. **No required/delivery date for now.** The order has only an `order_date` field.

### Scope

- **`shared_core`** — new `SalesOrder` model, `SalesOrderItem` model, `SalesOrderStatus` enum,
  `CreateSalesOrderRequest`, `SalesOrderListItem` DTO.
- **`backend`** — new DB layer, service layer, routes under `/api/sales-orders`; stock allocation/
  de-allocation logic; a "confirm" endpoint that converts the order to a `SalesInvoice`.
- **`frontend`** — new "New Sales Order" page (mirrors `new_sales_invoice.rs`), a "Sales Orders" list
  page, an order drawer with a Confirm button, plus stock-availability display on the line-item row.
- **`translations`** — new i18n keys in `en.ftl` and `fr.ftl`.

### Non-Goals

- Picking / Shipping / Fulfilment workflow (future).
- Purchase-order driven replenishment from a sales order.
- Quotation / pro-forma workflow.
- PDF printing of the order (can be added later following the invoice print pattern).
- Per-location picking location selection during confirm (future).

### Status Flow

## 2. How Prepayments & Workflows Work Seamlessly

With split statuses, accounting entries and inventory movements trigger independently based on real-world events:

### Scenario A: Prepayment (Pay First, Ship Later)
1. **Order Placed**:
  * `payment_status`: `Unpaid` | `fulfillment_status`: `Unfulfilled`
2. **Customer Pays upfront ($1,000)**:
  * `payment_status`: `Paid` | `fulfillment_status`: `Unfulfilled`
  * **GL Entry**: Debit `Cash at Bank` ($1,000) / Credit `Customer Prepayments / Unearned Revenue` ($1,000). *(Or directly to Accounts Receivable if using deposit clearing)*.
3. **Goods are Shipped**:
  * `fulfillment_status`: `Fulfilled`
  * **Stock Entry**: Issue physical inventory (Debit `COGS` / Credit `Inventory Asset`).
  * **GL Entry**: Recognize Revenue (Debit `Unearned Revenue` / Credit `Sales Revenue`).
4. **Overall Document Status**: Moves to `Completed`.

---

### Scenario B: B2B Credit (Ship First, Pay Later)
1. **Order Placed & Approved**:
  * `payment_status`: `Unpaid` | `fulfillment_status`: `Unfulfilled`
2. **Goods Shipped**:
  * `fulfillment_status`: `Fulfilled` | `payment_status`: `Unpaid`
  * **Stock Entry**: Debit `COGS` / Credit `Inventory Asset`.
  * **GL Entry**: Debit `Accounts Receivable` / Credit `Sales Revenue`.
3. **Customer Pays Invoice (30 Days Later)**:
  * `payment_status`: `Paid`
  * **GL Entry**: Debit `Cash at Bank` / Credit `Accounts Receivable`.
4. **Overall Document Status**: Moves to `Completed`.

---

### Scenario C: Pure Services (No Shipping Needed)
1. **Service Invoice Created**:
  * Set `fulfillment_status` = `NotRequired`.
2. **Service Approved / Issued**:
  * **GL Entry**: Debit `Accounts Receivable` / Credit `Service Revenue`.
3. **Customer Pays**:
  * `payment_status`: `Paid`.
4. **Overall Document Status**: Automatically moves to `Completed`.
---

## Sub-Tasks

---

### Sub-Task 1 — Shared Core: Sales Order Types

**Status:** [x] done

**Intent**  
Define all shared types needed by both the backend and the frontend. These must be in `shared_core`
so neither layer duplicates them.

**Expected Outcomes**
- `SalesOrderStatus` enum: `Open`, `Confirmed`, `Cancelled`.
- `SalesOrder` struct mirroring `SalesInvoice` (id, org_id, partner_id, warehouse_id, order_number,
  order_date, status, subtotal, tax_total, total_amount, billing/shipping address snapshot fields,
  `lines: Vec<SalesOrderItem>`). No `required_date` field.
- `SalesOrderItem` struct mirroring `SalesInvoiceItem` (id, order_id, item_id, code, name, description,
  quantity, unit_price, tax_category_id, tax_rate, tax_amount, net_amount, sort_order).
  Add `quantity_available: Option<Decimal>` — a computed/display field populated at read time (not stored).
- `SalesOrderListItem` DTO for the list view (id, order_number, partner name, order_date, warehouse name,
  status, total_amount).
- `CreateSalesOrderRequest` — fields: partner_id, warehouse_id, order_date, lines, address snapshots/ids.
  No `required_date` field.
- A `calculate()` method on `SalesOrder` (identical logic to `SalesInvoice::calculate()`).
- All types gated under `#[cfg(feature = "sales")]` (sales orders are part of the sales feature).

**Todo List**
1. Create `shared_core/src/sales/models/sales_order.rs` — `SalesOrder` struct + `calculate()`.
2. Create `shared_core/src/sales/models/sales_order_item.rs` — `SalesOrderItem` struct.
3. Create `shared_core/src/sales/models/sales_order_status.rs` — `SalesOrderStatus` enum.
4. Create `shared_core/src/sales/dtos/sales_order_list_item.rs` — `SalesOrderListItem` DTO.
5. Create `shared_core/src/sales/requests/sales_order.rs` — `CreateSalesOrderRequest`.
6. Register all new modules in `shared_core/src/sales/models/mod.rs`,
   `shared_core/src/sales/dtos/mod.rs`, `shared_core/src/sales/requests/mod.rs`.

**Relevant Context**
- Mirror the patterns in `shared_core/src/sales/models/sales_invoice.rs` and `sales_invoice_item.rs`.
- `SalesOrderStatus` follows the same `sqlx::Type` + `serde` pattern as `InvoiceStatus` in
  `shared_core/src/sales/models/invoice_status.rs`.
- Use `#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]` on structs.
- `quantity_available` on `SalesOrderItem` must NOT have a `sqlx` column mapping (it is injected at
  service level); use `#[cfg_attr(feature = "backend", sqlx(default))]` or set it after the query.

---

### Sub-Task 2 — Database: Migration for Sales Orders

**Status:** [x] done

**Intent**  
Add the `sales_orders` and `sales_order_items` tables, add the `sales_order_status` enum, and extend the
`organization_sequences` seqtype enum to include `SalesOrder` for auto-numbering.

**Expected Outcomes**
- New migration file `backend/migrations/2026XXXXXXX_sales_orders.sql` (use today's date).
- `sales_order_status` PostgreSQL enum: `open`, `confirmed`, `cancelled`.
- `sales_orders` table: id (UUID PK), organization_id (FK), partner_id (FK),
  warehouse_id (UUID FK → warehouses, NOT NULL), order_number (text), order_date (date),
  status (sales_order_status), subtotal (NUMERIC 15,4), tax_total (NUMERIC 15,4),
  total_amount (NUMERIC 15,4), billing_address_id (UUID nullable), shipping_address_id (UUID nullable),
  bill_to_* columns (snapshot, identical to sales_invoices), ship_to_* columns (snapshot, identical
  to sales_invoices), created_at (timestamptz default now()). No `required_date` column.
- `sales_order_items` table: id (UUID PK), order_id (UUID FK → sales_orders), item_id (UUID FK → items),
  code (text), name (text), description (text nullable), quantity (NUMERIC 15,4), unit_price
  (NUMERIC 15,4), tax_category_id (UUID nullable), tax_rate (NUMERIC 15,4), tax_amount (NUMERIC 15,4),
  net_amount (NUMERIC 15,4), sort_order (int).
- `seq_type` enum extended with `sales_order` value (ALTER TYPE ... ADD VALUE).
- Indexes: `(organization_id)` on both tables; `(organization_id, order_number)` unique on
  `sales_orders`; `(order_id)` on `sales_order_items`.

**Todo List**
1. Create `backend/migrations/YYYYMMDDNN_sales_orders.sql`.
2. Add `CREATE TYPE sales_order_status AS ENUM (...)`.
3. Add `ALTER TYPE seq_type ADD VALUE 'sales_order'` (idempotent with `IF NOT EXISTS`).
4. Add `CREATE TABLE sales_orders (...)` matching the `sales_invoices` structure.
5. Add `CREATE TABLE sales_order_items (...)` matching the `sales_invoice_items` structure.
6. Add all indexes.

**Relevant Context**
- Copy the bill_to/ship_to snapshot column pattern from `sales_invoices` in
  `backend/migrations/2026053102_migration.sql`.
- Keep enum value names `snake_case` to match the existing `sqlx(rename_all = "snake_case")` pattern.

---

### Sub-Task 3 — Backend: DB Layer for Sales Orders

**Status:** [x] done

**Intent**  
Create the database access functions for sales orders, following the pattern in
`backend/src/sales/db/sales_invoice.rs`.

**Expected Outcomes**
- `backend/src/sales/db/sales_order.rs` with functions:
  - `create_draft_order(conn, request, org_id, order_number) → SalesOrder`
  - `insert_sales_order_line(conn, line, order_id) → SalesOrderItem`
  - `get_sales_order(conn, id, org_id) → SalesOrder` (with lines)
  - `list_sales_orders(conn, org_id, status_filter) → Vec<SalesOrderListItem>`
  - `update_sales_order_totals(conn, id, org_id, subtotal, tax_total, total) → ()`
  - `update_sales_order_status(conn, id, org_id, new_status) → ()`
  - `get_sales_order_items(conn, order_id) → Vec<SalesOrderItem>`
- Module registered in `backend/src/sales/db/mod.rs`.

**Todo List**
1. Create `backend/src/sales/db/sales_order.rs`.
2. Implement each function above using raw `sqlx::query` / `sqlx::query_as` (no macros).
3. `get_sales_order` must fetch the header then call `get_sales_order_items` and populate `lines`.
4. Register `pub mod sales_order;` in `backend/src/sales/db/mod.rs`.

**Relevant Context**
- Follow the exact query style in `backend/src/sales/db/sales_invoice.rs` (no `sqlx::query_as!` macros).
- For `quantity_available` on each line item, set it to `None` in DB functions; the service layer will
  populate it after fetching balances from `inventory::db::inventory::get_item_stock_balances`.

---

### Sub-Task 4 — Backend: Service Layer for Sales Orders

**Status:** [x] done

**Intent**  
Implement business logic for creating orders, stock allocation on confirmation, and conversion to invoice.

**Expected Outcomes**
- `backend/src/sales/services/sales_order_service.rs` with:
  - `create_order(pool, req, org_id) → SalesOrder` — generates order number via
    `get_next_sequence_number(SeqType::SalesOrder)`, resolves address snapshots, inserts header + lines,
    calculates + updates totals. Does NOT allocate stock (order is just `Open`).
  - `get_sales_order(pool, id, org_id) → SalesOrder` — fetches order, then for each `Stocked` line item
    calls `inventory::db::get_item_stock_balances` scoped to the order's `warehouse_id` and sets
    `quantity_available` on the line (total available across all locations in that warehouse).
  - `list_sales_orders(pool, org_id, status_filter: Option<SalesOrderStatus>) → Vec<SalesOrderListItem>`
    — when `status_filter` is `None`, defaults to returning only `Open` orders (hidden default).
  - `confirm_order(pool, id, org_id, user_id) → SalesInvoice` — the main conversion function:
    1. Load the order (must be `Open`).
    2. For each `Stocked` item line: call `inventory::db::adjust_allocated(+quantity)` scoped to the
       order's warehouse and log a `StockTransaction(Allocation, SalesOrder ref)`.
       Use the first available picking location in the warehouse for the allocation record.
    3. Call `sales_invoice_service::create_invoice_from_order(...)` to create a `SalesInvoice` directly
       with status `Open`, re-using all existing invoice creation logic (GL posting, AR journal entries).
    4. Update order status to `Confirmed`.
    5. Return the new `SalesInvoice`.
  - `cancel_order(pool, id, org_id) → ()` — sets status `Cancelled`; if order was `Confirmed`,
    de-allocates stock (adjust_allocated with negative delta) and logs `StockTransaction(Adjustment)`.
- Module registered in `backend/src/sales/services/mod.rs`.

**Todo List**
1. Create `backend/src/sales/services/sales_order_service.rs`.
2. Implement `create_order` — reuse `account_service::get_next_sequence_number` and
   `partner_db::get_partner_address` for snapshot resolution.
3. Implement `get_sales_order` with `quantity_available` injection for stocked lines.
4. Implement `confirm_order` wrapped in an ACID transaction.
5. Add a helper `create_invoice_from_order` in `sales_invoice_service.rs` (or call existing
   `create_invoice` directly from order data).
6. Implement `cancel_order`.
7. Register in `backend/src/sales/services/mod.rs`.

**Relevant Context**
- `adjust_allocated` is in `backend/src/inventory/db/inventory.rs`.
- `log_transaction` is in `backend/src/inventory/db/stock_transaction.rs`.
- `TransactionType::Allocation` and `ReferenceType::SalesOrder` are already defined in
  `shared_core/src/inventory/models/stock_balance.rs`.
- The existing `create_invoice` in `sales_invoice_service.rs` already handles GL journal entries,
  address snapshots, and totalling — call or adapt it rather than duplicating.
- `SeqType` is in the core sequences module; follow the same pattern as `SeqType::SalesInvoice`.

---

### Sub-Task 5 — Backend: Routes for Sales Orders

**Status:** [x] done

**Intent**  
Expose the sales order service via HTTP endpoints following the existing sales invoice route structure.

**Expected Outcomes**
- `backend/src/sales/routes/sales_orders.rs` with:
  - `GET  /api/sales-orders` (list, `UseSales` guard, optional `?status=` query param)
  - `GET  /api/sales-orders/<id>` (single order with `quantity_available` on lines, `UseSales`)
  - `POST /api/sales-orders` (create, `ManageSales` guard)
  - `POST /api/sales-orders/<id>/confirm` (confirm → returns the new `SalesInvoice`, `ManageSales`)
  - `POST /api/sales-orders/<id>/cancel` (`ManageSales`)
- Routes registered in `backend/src/sales/routes/mod.rs` and mounted in `backend/src/main.rs`.

**Todo List**
1. Create `backend/src/sales/routes/sales_orders.rs`.
2. Implement each route handler (delegate to service layer).
3. Add `pub mod sales_orders;` to `backend/src/sales/routes/mod.rs`.
4. Mount routes in `backend/src/main.rs` inside the `#[cfg(feature = "sales")]` block.

**Relevant Context**
- Follow the pattern in `backend/src/sales/routes/sales_invoices.rs` for guard usage and error handling.
- The `confirm` endpoint returns `Json<SalesInvoice>` so the frontend can redirect to the new invoice.

---

### Sub-Task 6 — Frontend: Sales Order Line Row Component

**Status:** [x] done

**Intent**  
Create a reusable line-item row component for the new order form that extends the existing
`SalesInvoiceItemRow` with a stock-availability badge.

**Expected Outcomes**
- `frontend/src/sales/components/sales_order_item_row.rs` — based on `sales_invoice_item_row.rs`.
- Shows the same fields: item search, quantity input, unit price, tax category selector.
- Adds a read-only "Available" badge next to the quantity field, showing `quantity_available` from
  the line data when present (displayed as green if quantity ≤ available, amber if insufficient,
  grey/dash if item is not stocked or availability unknown).
- Component registered in `frontend/src/sales/components/mod.rs`.

**Todo List**
1. Copy `sales_invoice_item_row.rs` to `sales_order_item_row.rs`.
2. Change the props to use `SalesOrderItem` instead of `SalesInvoiceItem`.
3. Add a `quantity_available: Option<Decimal>` prop and render the availability badge.
4. Register in `frontend/src/sales/components/mod.rs`.

**Relevant Context**
- `frontend/src/sales/components/sales_invoice_item_row.rs` is the source pattern.
- The badge uses existing CSS classes; no new CSS needed.
- `quantity_available` is passed down from the parent page state (populated when editing an existing
  order fetched from the API).

---

### Sub-Task 7 — Frontend: New Sales Order Page

**Status:** [x] done

**Intent**  
Create the order entry page — the primary user-facing feature — closely following
`frontend/src/sales/pages/new_sales_invoice.rs`.

**Expected Outcomes**
- `frontend/src/sales/pages/new_sales_order.rs` — a `NewSalesOrderPage` function component.
- Form fields: customer (progressive search), warehouse selector (dropdown from `GET /api/warehouses`,
  required), order date, billing/shipping address tabs (same tab UI as new sales invoice), line items
  using `SalesOrderItemRow`. No required-date field.
- When the warehouse changes, re-fetch stock availability for any already-entered stocked line items
  and update the availability badges.
- On save: `POST /api/sales-orders` → navigates to `Route::SalesOrders` (the list).
- No GL posting at this stage (order creation is not a financial event).
- Page registered in `frontend/src/sales/pages/mod.rs`.

**Todo List**
1. Create `frontend/src/sales/pages/new_sales_order.rs` modelled on `new_sales_invoice.rs`.
2. Replace `CreateSalesInvoiceRequest` with `CreateSalesOrderRequest`.
3. Add warehouse selector: fetch `GET /api/warehouses`, populate a `<select>`, bind to `warehouse_id`
   in the request state.
4. Replace `SalesInvoiceItemRow` with `SalesOrderItemRow`.
5. Change the API call to `POST /api/sales-orders`.
6. On success, navigate to `Route::SalesOrders` (the list page).
7. Register in `frontend/src/sales/pages/mod.rs`.

**Relevant Context**
- `frontend/src/sales/pages/new_sales_invoice.rs` is the primary pattern to follow.
- Address handling, progressive customer search, and line item calculation logic can be copied
  with minimal changes (just replace types).
- Translations for labels reuse keys wherever identical to invoice (e.g. `common-save`), with
  new keys for order-specific labels.
- Warehouse list is already fetched in inventory pages (e.g. `warehouse_list.rs`) — reuse that
  API call pattern.

---

### Sub-Task 8 — Frontend: Sales Orders List Page & Order Drawer

**Status:** [x] done

**Intent**  
Provide a list of all sales orders and a detail drawer (similar to the sales invoice drawer) with a
prominent **Confirm Order** button that converts the order to an invoice.

**Expected Outcomes**
- `frontend/src/sales/pages/sales_orders.rs` — `SalesOrdersPage`:
  - Table of orders from `GET /api/sales-orders` (default: Open only) showing: order number,
    customer, warehouse, order date, status chip, total amount.
  - A status filter control to optionally show Confirmed or Cancelled orders (passes `?status=`
    query param to the API).
  - "New Order" button navigates to `Route::NewSalesOrder`.
  - Clicking a row opens the order drawer.
- `frontend/src/sales/components/sales_order_drawer.rs` (single file, following the simpler
  invoice drawer pattern):
  - Shows order header (order number, customer, warehouse, order date, status chip).
  - Address snapshot tabs (billing / shipping).
  - Line items table with availability badges (loaded from `GET /api/sales-orders/<id>` which
    injects `quantity_available`).
  - **Confirm Order** button (visible when status = `Open`, requires `manage_sales` privilege) →
    `POST /api/sales-orders/<id>/confirm` → on success, navigate to `Route::SalesLedger` so the
    user can see the newly created invoice.
  - **Cancel Order** button (visible when status = `Open`, requires `manage_sales` privilege).
  - Status chip: Open (blue), Confirmed (green), Cancelled (grey).
- Pages/components registered in their respective `mod.rs` files.

**Todo List**
1. Create `frontend/src/sales/pages/sales_orders.rs`.
2. Create `frontend/src/sales/components/sales_order_drawer.rs` (or sub-directory).
3. Implement the table, status chips, and navigation.
4. Implement the Confirm and Cancel actions with API calls and error handling.
5. Register page in `frontend/src/sales/pages/mod.rs`.
6. Register component in `frontend/src/sales/components/mod.rs`.

**Relevant Context**
- The existing `sales_invoice_drawer` in `frontend/src/sales/components/sales_invoice_drawer/` shows
  the component-per-tab pattern to follow.
- Status chip styling can reuse patterns from the invoice status display.

---

### Sub-Task 9 — Frontend: Router & Navigation

**Status:** [x] done

**Intent**  
Wire up the new pages to the application router and navigation sidebar.

**Expected Outcomes**
- Two new routes in `frontend/src/router.rs` (both gated on `feature = "sales"`):
  - `Route::SalesOrders` → `/sales/orders`
  - `Route::NewSalesOrder` → `/sales/orders/new`
- The switch in `frontend/src/main.rs` (or wherever routes are matched to components) maps these to
  `SalesOrdersPage` and `NewSalesOrderPage`.
- The sidebar/navigation includes a "Sales Orders" link under the Sales section, visible when the
  user has `use_sales` privilege.

**Todo List**
1. Add `SalesOrders` and `NewSalesOrder` variants to `Route` enum in `frontend/src/router.rs`.
2. Add the matching arms in the route switch (following the `NewSalesInvoice` pattern).
3. Add the navigation link in the sidebar component (find the Sales section).

**Relevant Context**
- `frontend/src/router.rs` shows all existing routes.
- Navigation sidebar is likely in `frontend/src/core/components/` — find the component that renders
  the sales nav links to place the new item alongside them.

---

### Sub-Task 10 — Translations

**Status:** [x] done

**Intent**  
Add all new i18n keys for order-specific strings so no raw English text appears in the UI.

**Expected Outcomes**
- `shared_core/translations/en.ftl` extended with keys for:
  - Page title, headers, button labels (new-sales-order-*, sales-order-list-*, sales-order-drawer-*).
  - Status labels: `sales-order-status-open`, `sales-order-status-confirmed`, `sales-order-status-cancelled`.
  - Availability badge: `sales-order-item-available`, `sales-order-item-insufficient-stock`.
  - Error / success messages for create, confirm, cancel.
- `shared_core/translations/fr.ftl` extended with matching keys (can use English text as placeholder
  values pending proper translation).

**Todo List**
1. Audit all new UI strings introduced in Sub-Tasks 6–9.
2. Add corresponding keys to `shared_core/translations/en.ftl`.
3. Add matching keys to `shared_core/translations/fr.ftl`.

**Relevant Context**
- Follow the naming convention of existing sales keys in `en.ftl` (e.g. `new-sales-invoice-title`,
  `sales-invoice-drawer-inv-number`).
- The `i18n.t("key")` and `i18n.t_args("key", &fluent_args![...])` pattern is used in all frontend
  components.
