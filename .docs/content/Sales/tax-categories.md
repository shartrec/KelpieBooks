+++
title = "Tax Categories and Rates"
description = "How to configure and maintain tax categories and rates."
weight = 10
+++

# Tax Categories and Rates

Tax Categories allow you to define different tax classifications (e.g., standard sales tax, tax-free, or special reduced rates) and specify their rates over time. These categories are assigned to catalog items to automatically compute tax amounts during sales invoicing.

{% screenshot() %}
![Tax Categories List Screenshot](../../screenshots/sales/tax-categories.png)
{% end %}

## Adding a Tax Category

1. Navigate to the **Tax Categories** page under the **Sales** menu.
2. Click the **+ Add Tax Category** button (located in the top right).
3. In the modal that appears, enter the following:
   - **Name**: The name of the tax category (e.g., "Standard VAT" or "GST").
   - **Description**: An optional brief description.
   - **Is Active**: Check this box to make the tax category available for catalog items.
4. Click **Save** to create the category.

## Managing a Tax Category

Select any tax category from the list table to open its **Tax Category Drawer**. The drawer contains two tabs: **General** and **Manage Rates**.

### General Tab

The **General** tab allows you to edit the basic profile details of the tax category.

{% screenshot() %}
![Edit Tax Category - General Tab Screenshot](../../screenshots/sales/edit-tax-category-general.png)
{% end %}

- Update the **Name**, **Description**, or toggle the **Is Active** checkbox.
- Click **Save** to apply changes.

### Manage Rates Tab

A tax category can have multiple tax rates, each applicable within a specific validity period. This ensures historical invoices remain accurate even if tax rates change over time.

{% screenshot() %}
![Edit Tax Category - Rates Tab Screenshot](../../screenshots/sales/edit-tax-category-rates.png)
{% end %}

#### Adding a Tax Rate
1. Click the **+ Add Rate** button in the **Manage Rates** tab.
2. In the edit card that appears, configure the rate details:
   - **Name**: A descriptive name for the rate (e.g., "GST 15%").
   - **Rate**: The percentage value of the tax rate (e.g., `15.0000`).
   - **Account**: Select the Liability Account to which the collected tax should be allocated.
   - **Valid From**: The date the tax rate becomes active.
   - **Valid To**: Optional end date for the tax rate.
3. Click **Save** on the edit card.
4. Click **Save** at the bottom of the tab to save the rate list.

#### Editing or Deleting a Tax Rate
- Click the **Edit (Pencil Icon)** on any rate card to modify its name, rate, liability account, or validity range.
- Click the **Delete (Trash Icon)** on any rate card to remove it. Confirm the deletion in the modal.
- Remember to click **Save** at the bottom of the tab to persist your changes.
