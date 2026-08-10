Here is a structured architectural plan to integrate double-entry General Ledger (GL) accounting into your inventory system.

---

## 1. Core Accounting Model for Inventory

To maintain double-entry integrity, **every physical movement must trigger a balanced Journal Entry** (

$$\sum \text{Debits} = \sum \text{Credits}$$

).

### General Ledger Account Requirements

You will need the following accounts in your Chart of Accounts (COA):

1. **Inventory Asset Account** *(Asset - Balance Sheet)*: Tracks the monetary value of stock on hand.
2. **AP Clearing / Received Not Invoiced (RNI)** *(Liability - Balance Sheet)*: Holding account for received goods prior to vendor bill/invoice processing.
3. **Inventory Adjustment Account** *(Expense/Loss or Revenue/Gain - P&L)*: Records gains/losses from stock counts, damage, or scrap.
4. **Cost of Goods Sold (COGS)** *(Expense - P&L)*: Expensed when inventory is sold or delivered.

---

## 2. Valuation Strategy

Before posting monetary amounts, you must define how inventory unit cost is determined.

* **Weighted Average Cost (WAC) (Recommended for simplicity):**

$$\text{New Unit Cost} = \frac{(\text{Existing Value} + \text{Received Value})}{\text{Total New On-Hand Quantity}}$$


* **Standard Costing:** Pre-defined static cost; variances go to a Purchase Price Variance (PPV) GL account.
* **FIFO / LIFO / Specific Identification (Lot/Serial):** Requires tracking individual inventory layers/batches.

*Assuming **Weighted Average Costing** or **Standard Costing** for the baseline implementation.*

---

## 3. Transaction Mapping Matrix

Here is how physical movements translate into GL Journal Entries:

### A. Receiving Stock (Purchase Receipt)

When goods arrive in the warehouse before an invoice is entered:

* **Debit:** Inventory Asset Account *(Increases Asset)*
* **Credit:** Received Not Invoiced (RNI) / AP Clearing *(Increases Liability)*

> *Note: When the vendor bill eventually arrives, AP clears RNI:*
> *Debit: RNI | Credit: Accounts Payable*

---

### B. Stock Adjustments

#### 1. Positive Adjustment (Found Stock / Cycle Count Gain)

* **Debit:** Inventory Asset Account *(Increases Asset)*
* **Credit:** Inventory Adjustment Account *(Decreases Expense / Increases Gain)*

#### 2. Negative Adjustment (Damage / Scrap / Cycle Count Loss)

* **Debit:** Inventory Adjustment Account / Scrap Expense *(Increases Expense)*
* **Credit:** Inventory Asset Account *(Decreases Asset)*

---

### C. Inventory Transfers (Between Warehouses)

* If moving between locations within the same legal entity/GL segment: **No GL Impact** (only physical location updates).
* If moving across GL segments/entities:
* **Debit:** Destination Inventory Asset
* **Credit:** Source Inventory Asset



---

## 4. System Implementation Architecture

```
┌─────────────────────────┐
│ Physical Movement Event │ (Receive / Adjust)
└────────────┬────────────┘
             │
             ▼
┌─────────────────────────┐
│ Inventory Costing Engine│ (Calculates unit_cost & total_value)
└────────────┬────────────┘
             │
             ▼
┌─────────────────────────┐
│  GL Posting Service     │ (Generates & posts Journal Entry)
└────────────┬────────────┘
             │
             ├───────────────────────┐
             ▼                       ▼
┌─────────────────────────┐ ┌─────────────────────────┐
│ journal_entries         │ │ journal_entry_lines     │
└─────────────────────────┘ └─────────────────────────┘

```

---

## 5. Phased Execution Plan

### **Phase 1: DB Schema & Data Model Setup**

1. Ensure items have cost fields (`unit_cost`, `costing_method`).
2. Add default GL Account mappings to Item Categories or System Settings:
* `inventory_account_id`
* `cogs_account_id`
* `adjustment_account_id`
* `rni_account_id`



### **Phase 2: Valuation Calculation Service**

1. Implement a module in `shared_core` to compute total movement value (`quantity_delta * unit_cost`).
2. Update item unit costs on receipt events (if using Weighted Average Costing).

### **Phase 3: Automated Journal Posting Service**

1. Create a service (`post_inventory_journal`) inside your backend transaction context.
2. Link `inventory_transaction_id` or `receipt_id` to the generated `journal_entry_id` for auditability/traceability.

### **Phase 4: Safety & Reversion Logic**

1. Wrap physical stock changes and GL postings in a **single database transaction** (`sqlx::Transaction`). If GL posting fails, the stock movement rolls back.
2. Handle cancellation/reversals by creating reverse journal entries rather than deleting history.

---

### Next Steps

To begin implementation, we can tackle **Phase 1** (defining GL account mappings on items/categories) or **Phase 2 & 3** (building the SQL transaction that creates the journal entry on stock receipt).

Which valuation method (Weighted Average, Standard, or FIFO) fits your system design best?

Ans: Weighted Avg at this stage

## Unit costs

When calling `process_stock_movement(...)`, where you source the `unit_cost` depends on the **transaction type** (whether stock is entering, exiting, or being adjusted) and what input data is available.

Here is the standard pattern for sourcing `unit_cost`:

---

### 1. Stock Receiving (`StockTransactionType::Receive`)

* **Source Primary**: User input from the Receiving Modal (e.g., vendor invoice or purchase order unit price).
* **Source Fallback**: `item.purchase_unit_cost` (pre-filled on the modal UI so the user can override it if the cost changed for this specific batch).

```rust
// Received cost from payload or fall back to item master standard/purchase cost
let unit_cost = input.unit_cost.unwrap_or(item.purchase_unit_cost);
```

---

### 2. Stock Adjustments (`StockTransactionType::Adjustment`)

#### **For Stock Loss (Negative Adjustment: -Qty)**

Always valuation-based. You must use the **current inventory valuation cost** from `warehouse_inventory_balances.unit_cost` for that location/item so you expense the exact asset value being removed.

#### **For Stock Gain (Positive Adjustment: +Qty)**

* **If current stock exists**: Use the current `warehouse_inventory_balances.unit_cost`.
* **If location balance is 0/empty**: Fall back to `item.purchase_unit_cost`.

```rust
// Look up current balance valuation cost
let current_balance = get_warehouse_balance(tx, org_id, warehouse_id, location_id, item_id).await?;

let unit_cost = match current_balance {
    Some(b) if b.unit_cost > Decimal::ZERO => b.unit_cost,
    _ => item.purchase_unit_cost, // Fallback when initial balance has 0 cost
};

```

---

### 3. Stock Transfers (`StockTransactionType::Transfer`)

Always source the `unit_cost` from the **source warehouse location's** `warehouse_inventory_balances.unit_cost`. Transfers must move inventory at its current carrying value so no artificial GL gain/loss is recognized.

---

### Summary Matrix

| Transaction Type | Primary Source | Fallback Source |
| --- | --- | --- |
| **Receive** | Payload `unit_cost` (UI input) | `item.purchase_unit_cost` |
| **Adjustment (Loss)** | `warehouse_inventory_balances.unit_cost` | `item.purchase_unit_cost` |
| **Adjustment (Gain)** | `warehouse_inventory_balances.unit_cost` | `item.purchase_unit_cost` |
| **Transfer** | Source `warehouse_inventory_balances.unit_cost` | `item.purchase_unit_cost` |
| **Sale / Issue** | `warehouse_inventory_balances.unit_cost` (COGS) | `item.purchase_unit_cost` |