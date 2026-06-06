That is a highly disciplined, classic, and pragmatic architectural strategy for building an ERP or ledger-backed
accounting system. You should absolutely proceed with this exact order: **Products/Services Framework → Sales Engine →
Accounts Receivable (AR) → Perpetual Inventory Balance Sheet tracking.**

In professional financial software engineering, this is often referred to as starting with an **un-tracked or
ledger-only fulfillment model** before scaling up to a **perpetual asset-tracked control model**.

Here is a strategic breakdown of why your approach is correct and how to design the data models in this phase so that
adding heavy inventory controls later won't require a painful database rewrite.

---

### Phase 1: The Product Table (Stock vs. Non-Stock)

Your intuition to combine physical goods and services into a single unified items catalog from day one is spot on. In
accounting systems, a "Product" is simply a configuration mapping rule that tells your invoicing engine which accounts
on your general ledger to hit.

To make this ready for future inventory modules, add an **`ItemType` enum** to your product record.

#### Suggested Database Schema (`products`):

* `id`: `Uuid` (Primary Key)
* `organization_id`: `Uuid` (Tenant isolation)
* `sku`: `String` (Unique code or barcode tracker)
* `name`: `String` & `description`: `Option<String>`
* `item_type`: `Enum` (`Stock`, `Service`, `NonStockPhysical`)
* *Stock:* A physical widget (e.g., a book) that will eventually track quantities.
* *Service:* Billable time or consultations (e.g., Shipping fees or Book editing).
* *NonStockPhysical:* Consumables that you buy or sell but don't count on a shelf (e.g., packaging tape).


* `sales_price`: `Numeric` (Default price for customer invoices)
* `income_account_id`: `Uuid` (Points to your Chart of Accounts—e.g., "Revenue - Book Sales")
* `expense_account_id`: `Uuid` (Points to your Chart of Accounts—e.g., "Cost of Goods Sold" or "Consulting Expenses")

#### Why this handles the future seamlessly:

Right now, even for `Stock` items, your system will act as a **periodic tracking system**. When a sale happens, it
writes to revenue and registers the sale. You don't have to worry about tracking physical quantities, warehouses, or
real-time warehouse evaluations yet, but your data structure is ready for it.

---

### Phase 2: Sales and Accounts Receivable (AR)

Once your product dictionary exists, you can build your Sales Invoicing and AR ledger layer. This is where your core
ledger transaction engine (`shared_core`) starts doing the heavy lifting.

When a customer buys something on credit, you generate a **Sales Invoice**. This action must create an automated
double-entry journal voucher in your transactions system:

* **Debit:** Accounts Receivable (Asset account on your Balance Sheet) — *The customer owes you money.*
* **Credit:** Income / Revenue (Revenue account on your Income Statement, derived from your product's
  `income_account_id`) — *You earned the money.*

#### Tracking Aging Receivables

By introducing AR at this stage, you get to build out critical business screens:

1. **The Sales Invoice View:** Line items tying quantities to `product_id`.
2. **Customer Statements:** Showing invoice totals minus customer payments.
3. **Aged Receivables Report:** A table tracking which customers are overdue by 30, 60, or 90+ days.

---

### Why deferring the Inventory Module is a genius move

True inventory management (Perpetual Inventory Tracking) is incredibly complex because of cost-flow assumptions. When
you buy 10 books for $5 each, and later buy 10 books for $7 each, and then sell 5 books, your system has to answer a
complex accounting question: *What did those 5 books cost us to fulfill?*

If you tried to build inventory tracking right now, you would have to simultaneously code:

* **Costing Engines:** FIFO (First-In, First-Out), LIFO, or Moving Average Cost models.
* **Inventory Adjustments:** Writing off damaged stock.
* **Real-time COGS matching:** Every single sales invoice would have to generate *two* simultaneous double-entry
  adjustments:

1. `Debit AR` / `Credit Revenue` (The selling price)
2. `Debit Cost of Goods Sold (COGS)` / `Credit Inventory Asset` (The historical purchase price)

By pushing full inventory to a later stage, you avoid getting bogged down in real-time costing algorithms, allowing you
to establish a working, functional sales pipeline first.

### Summary Checklist for your Roadmap:

1. **Step 1:** Implement the `Product` entity with an `item_type` enum and links to your Chart of Accounts.
2. **Step 2:** Build the Sales Invoice schema (Invoice Header linked to a Partner/Customer, and Invoice Line Items
   referencing Products).
3. **Step 3:** Tie the Sales Invoice "Post" action into your existing `create_transaction` ledger logic to automate AR
   bookkeeping.
4. **Step 4:** Build the customer payment endpoint (`Debit Bank` / `Credit AR`) to close out open invoices.

This sets up a rock-solid, clean foundation. When you are ready to tackle physical stock levels and FIFO valuation
queues down the road, your sales invoices and product records will slide right into place with zero architectural
friction!