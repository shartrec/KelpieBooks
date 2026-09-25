+++
title = "Warehouse Locations"
description = "Managing warehouse location structures, zones, aisles, and storage bins."
weight = 13
+++

# Warehouse Locations

{% <warning> %}
This feature and page are currently under active development.
{% </warning> %}

The **Warehouse Locations** page provides a comprehensive breakdown of the internal storage topography for a specific warehouse facility. It allows inventory managers to structure, organize, and filter storage slots into zones, aisles, shelves, and bins.

{{< screenshot src="../../screenshots/inventory/warehouse-location-list.png" alt="Warehouse Locations Screenshot" />}}

## Navigation & Warehouse Structure

The page is split into a navigation sidebar and a main location grid:

### Warehouse Structure Sidebar

The **Warehouse Structure** tree sidebar on the left displays the hierarchical topography of the warehouse:
- **All Storage Locations**: Displays all locations defined within the warehouse.
- **Zones & Aisles**: Expand any zone node (e.g., *Electrical*, *Paint*) to view its constituent aisles. Clicking a zone or aisle filters the grid view to show only the locations within that structural area.

### Location Grid

The main pane displays the filtered list of locations with the following details:

- **Location Tag**: The full formatted location code (e.g., `Electrical-10-A-1`).
- **Zone**: The high-level zone or section of the warehouse.
- **Aisle**: The specific aisle identifier.
- **Shelf / Tier**: The shelf level or tier height within the rack.
- **Bin / Slot**: The specific bin or slot position on the shelf.
- **Picking Loc**: Indicates whether this location is designated as an active picking location for order fulfillment.

## Managing Locations

### Bulk Generating Locations

To create multiple location tags at once, click the **Bulk Generate Locations** button at the top of the grid.

- For detailed instructions, see the [Bulk Location Generator](../bulk-location-generator/) page.

