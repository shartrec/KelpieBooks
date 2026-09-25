+++
title = "Bulk Location Generator"
description = "How to bulk generate warehouse storage locations and slotting matrices."
weight = 14
+++

# Bulk Location Generator

The **Bulk Matrix Generator** allows inventory administrators to rapidly generate matrix combinations of storage locations for a warehouse zone.

You can open this tool from the [Warehouse Locations](../warehouse-locations/) page by clicking the **Bulk Generate Locations** button above the location grid.

{{< screenshot src="../../screenshots/inventory/bulk-location-gen.png" alt="Bulk Matrix Generator Screenshot" />}}

## Generator Fields

Configure the matrix generation parameters:

- **Target Zone Name**: Enter the zone prefix for the generated locations (e.g., `Bulk B` or `Electrical`).
- **Aisle Sequence Range**: Set the starting and ending numeric aisle numbers (e.g., `1` to `5`).
- **Shelf/Tier Sequence Range (Alpha)**: Set the starting and ending alphabetic shelf tier levels (e.g., `A` to `D`).
- **Bin/Slot Sequence Range**: Set the starting and ending numeric bin/slot position numbers (e.g., `1` to `2`).
- **Designate generated rows as Picking Locations**: Check this option to mark all generated locations as active picking slots for order fulfillment.

## Executing Generation

- Click **Execute Bulk Generation** to create all combination tags matching the `{zone}-{aisle}-{shelf}-{bin}` naming format.
- Click **Cancel** to discard inputs and exit the modal dialog.
