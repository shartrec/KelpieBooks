+++
title = "Catalog Items"
description = "Managing catalog items, prices, and stock balances."
weight = 1
+++

# Catalog Items

Catalog Items represent the goods or services your organization sells or manages in inventory. Maintaining a centralized item catalog allows you to track stock levels, set consistent pricing, and manage fulfillment operations across your warehouses.

{{< screenshot src="../../screenshots/inventory/item-list.png" alt="Catalog Items List Screenshot" />}}

## Main View & Stock Breakdown

The table on this page displays all configured items with the following columns:

- **Code**: A unique identifier for the item (e.g., `P189555-11`).
- **Name**: The descriptive name of the item.
- **Type**: The item classification (**Stocked**, **Non-Stocked**, or **Service**).
- **Price**: The standard unit selling price.
- **Unit Cost**: The unit cost price for inventory valuation.
- **Actions**: Controls for editing item details, receiving inventory, or submitting stock adjustments.

### Expandable Location Balances

For **Stocked** items, click the expand icon (`>`) next to the item code to reveal a location-level stock balance breakdown table:

- **Warehouse**: The warehouse facility holding the stock.
- **Location / Bin**: The specific storage bin or location tag within the warehouse (e.g., `Electrical-10-A-3`).
- **On Hand**: Total physical stock recorded at this location.
- **Allocated**: Quantity reserved for open orders.
- **Available**: Calculated net available stock (`On Hand` − `Allocated`).

## Searching and Filtering Items

Use the filter bar at the top of the table to locate specific items:
- **Search**: Enter text to filter items by code or name.
- **Type Filter**: Filter by "All Types," "Service," "Stocked," or "Non-Stocked."
- **Include Inactive**: Check to include discontinued or retired items in the list.
- **Items Per Page**: Adjust the dropdown to control table row count.

## Adding a Catalog Item

1. Click the **Add Item** button above the table.
2. In the modal form, configure the item details:
   - **Code**: A unique identifier for the item (e.g., `PROD-001`).
   - **Name**: The display name of the item.
   - **Description**: An optional detailed description.
   - **Type**: Select from:
     - **Service**: Non-tangible services (e.g., consulting, labor).
     - **Stocked**: Physical inventory tracked by location balances.
     - **Non-Stocked**: Physical goods not tracked in inventory balances.
   - **Unit of Measure**: Select the unit (e.g., `Each`, `Hour`).
   - **Price**: The standard unit selling price.
   - **Income Account**: Select the Revenue account for sales postings.
   - **Tax Category**: Select the predefined tax category for invoicing.
3. Click **Save** to add the item to the catalog.

{{< screenshot src="../../screenshots/inventory/add-item.png" alt="Add Catalog Item Modal Screenshot" />}}

## Managing Inventory & Item Actions

### Receiving Stock

Click the **Receive Stock** icon (box icon) on an item row or location sub-row to record incoming stock:
- Select the target **Warehouse** and **Location / Bin**.
- Enter the optional **PO Number** and **Quantity** received.
- Click **Post Receipt** to commit stock to the ledger.

### Stock Adjustments

Click the **Adjust Stock** icon (sliders icon) on an item row or location sub-row to post stock level adjustments:
- Select the target **Warehouse** and **Location / Bin**.
- Choose an **Adjustment Reason** (Cycle Count, Damage, Scrap, Audit Correction, Found Stock, or Other).
- Enter the **Quantity Delta** (positive to increase, negative to decrease) and optional **Notes**.
- Click **Commit Adjustment** to apply the inventory change.

### Editing or Disabling Items

- **Edit**: Click the **Edit (Pencil Icon)** on an item row to update its code, name, price, tax category, or active status.
- **Delete**: Click the **Delete (Trash Icon)** on a row to remove an item. You can only delete catalog items that have no transaction history.
