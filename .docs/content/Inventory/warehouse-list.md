+++
title = "Warehouse List"
description = "Managing your warehouses from the Warehouse List screen."
weight = 10
+++

# Warehouses

Warehouses represent physical inventory distribution nodes, fulfillment hubs, and stock storage locations within your organization. Maintaining a structured warehouse directory allows you to track stock levels, organize storage locations, and manage inventory operations effectively.

{{< screenshot src="../../screenshots/inventory/warehouse-list.png" alt="Warehouse List Screenshot" />}}

The main Warehouse List screen provides a centralized view of all configured warehouses in your system.

## Main View

The table on this page displays all your warehouses with the following columns:

- **Code**: A unique identifier or code for the warehouse.
- **Name**: The descriptive name of the warehouse.
- **Active**: A checkbox indicating whether active stock tracking is enabled for the warehouse.
- **Actions**: A set of controls for managing the warehouse.

## Managing Warehouses

You can manage your warehouses directly from this screen.

### Adding a New Warehouse

To create a new warehouse, click the **Add Warehouse** button located at the top of the page.

- For detailed instructions, see the [Edit Warehouse](../edit-warehouse/) page.

### Editing an Existing Warehouse

To modify a warehouse's details (such as its name, code, or active status), click the **Edit** button in the "Actions" column for that warehouse row.

- For detailed instructions, see the [Edit Warehouse](../edit-warehouse/) page.

### Viewing Warehouse Locations

To view and manage the internal locations, aisles, shelves, or storage bins defined within a warehouse, click the **View Location Mapping Structure** button (eye icon) in the "Actions" column.

- For detailed instructions, see the [Warehouse Locations](../warehouse-locations/) page.

### Deleting a Warehouse

To remove a warehouse, click the **Delete** button in the "Actions" column. Please note that you cannot delete warehouses that still contain nested storage locations or active tracking balances. This is a safety measure to prevent accidental data loss.
