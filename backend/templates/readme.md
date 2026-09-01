# Typst Document Templates

This directory contains the Typst template definitions (`.typ`) and local package dependencies used by the backend engine to compile customer-facing PDFs (Invoices, Picklists, Packing Slips, etc.).

These templates can be easily modifies to suit your needs. See https://typst.app/docs/
## Directory Layout

```text
templates/
├── invoice_template.typ     # Invoice PDF template
├── picklist_template.typ    # Warehouse Picking Slip PDF template
├── report_template.typ      # General report template
├── service.toml             # Service configuration
└── typst/
    └── packages/
        └── local/           # Vendored Typst packages & WASM plugins (e.g., zebra)

```

---

## Data Contract (`sys.inputs`) for Sales Order Templates, invoice & picklists

When rendering documents via the backend, data is injected into Typst as a nested dictionary available through `sys.inputs`.

All monetary values and quantities are serialized as pre-formatted strings to ensure localized currency and number formatting are consistent with the user's locale settings.

### Top-Level Keys

| Key | Type | Description | Example |
| --- | --- | --- | --- |
| `company-name` | String | Trading/Legal name of the issuing company | `"Trevor's Toy Emporium"` |
| `order-id` | String (UUID) | Database primary key (`OrderId`) | `"a0012f2c-44bb-47ca-b786-820a224e993b"` |
| `order-number` | String | Sequential document reference number | `"1008"` |
| `order-date` | String | Formatted date of order creation | `"21 Aug 2026"` |
| `due-date` | String | Formatted payment due date | `"21 Aug 2026"` |
| `order-net` | String | Subtotal before taxes | `"350.70"` |
| `order-tax` | String | Total calculated tax | `"40.33"` |
| `order-gross` | String | Grand total inclusive of tax | `"391.03"` |
| `bill_to` | Dictionary | Billing address structure *(see Address Schema)* | `{ ... }` |
| `ship_to` | Dictionary | Delivery address structure *(see Address Schema)* | `{ ... }` |
| `lines` | Array[Dict] | List of document line items *(see Line Schema)* | `[ { ... } ]` |

---

### Address Schema (`bill_to` / `ship_to`)

| Key | Type | Description | Example |
| --- | --- | --- | --- |
| `name` | String | Partner / Business name | `"Barbeque Man"` |
| `attn` | String | Recipient / Attention line (optional) | `""` |
| `addr_line1` | String | Primary street address | `"123 Meatlovers Ln"` |
| `addr_line2` | String | Secondary street address (optional) | `""` |
| `city` | String | City / Suburb | `"Meatville"` |
| `state` | String | State / Region code | `"NSW"` |
| `post_code` | String | Postal / Zip code | `"2598"` |

---

### Line Item Schema (`lines[]`)

| Key | Type | Description | Example |
| --- | --- | --- | --- |
| `name` | String | Item name or line description | `"Wireless Gizmo"` |
| `code` | String | SKU or product code (optional) | `""` |
| `qty` | String | Ordered quantity | `"1"` |
| `uom` | String | Unit of Measure | `"Each"` |
| `unit_price` | String | Net unit price | `"350.70"` |
| `net` | String | Line net total (`qty * unit_price`) | `"350.70"` |
| `tax` | String | Line tax total | `"40.33"` |
| `gross` | String | Line total inclusive of tax | `"350.70"` |
| `location` | String | Primary warehouse storage bin location | `"Electrical-10-A-1"` |

---

## Data Contract (`sys.inputs`) for General Reports

The `report_template.typ` provides a consistent page layout with header and footer across the application.

### Top-Level Keys

| Key                | Type | Description                                    | Example                                  |
|--------------------| --- |------------------------------------------------|------------------------------------------|
| `title`            | String | The report title                               | `"Profit and Loss"`                      |
| `report_qualifier` | String | Generally the selection criteria of the report | `"Period 30 Jan 2025 - 31 Dec 2026"` |
| `org_name`         | String | Trading/Legal name of the issuing company      | `"Trevor's Toy Emporium"`                |

## Accessing Packages & Assets

Templates utilize a file-system resolver mounted at the `templates/` directory root.

### Vendored Packages

Local packages (including compiled WASM plugins such as `zebra`) should be imported using relative paths starting from the `templates/` root:

```typst
#import "typst/packages/local/zebra/0.1.0/lib.typ": *

// Render a QR code for mobile picking/scanners
#qrcode(sys.inputs.at("order-number"), width: 2cm)

```

### Accessing Inputs Safely

Use `.at("key", default: ...)` when reading optional attributes to avoid compilation errors if optional fields (like `code` or `attn`) are empty strings:

```typst
#let data = sys.inputs
#let ship = data.at("ship_to")

#text(weight: "bold")[#data.at("order-number")]
#text()[#ship.at("name")]

```
