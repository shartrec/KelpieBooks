##
## Fluent Translation File for KelpieBooks (en)
##

## English is the master source for translation keys. Other languages should match this file layout

# Privileges
# Organization Administrator Role (Bootstrap)
sys-privilege-security_admin-name = Organization Administrator
sys-privilege-security_admin-description = Root administration role with full, uninhibited configuration and data modification access across the entire organization cluster.

# Accounts Module
sys-privilege-use_accounts-name = View Accounts
sys-privilege-use_accounts-description = Permits viewing the Chart of Accounts, tracking operational account histories, ledger summaries, and real-time balances.

sys-privilege-manage_accounts-name = Manage Accounts
sys-privilege-manage_accounts-description = Permits creating, rewriting parameters, re-structuring, or deactivating entries in the general Chart of Accounts.

# Partners Module
sys-privilege-use_partners-name = View Partners
sys-privilege-use_partners-description = Permits viewing profiles, contact parameters, and transaction histories for vendors, clients, and counterparties.

sys-privilege-manage_partners-name = Manage Partners
sys-privilege-manage_partners-description = Permits initializing new partner files, rewriting metadata details, or safely soft-deleting partner records.

# Payables Module
sys-privilege-use_vendor_invoices-name = View Vendor invoices
sys-privilege-use_vendor_invoices-description = Permits viewing vendor invoices.

sys-privilege-manage_vendor_invoices-name = Manage Vendor Invoices
sys-privilege-manage_vendor_invoices-description = Permits entering, updating and paying of vendor invoices.

# Sales Module
sys-privilege-use_sales-name = View item and sales invoices
sys-privilege-use_sales-description = Permites view sales invoices and item details.

sys-privilege-manage_sales-name = Mangage items and sales invoices
sys-privilege-manage_sales-description = Permits maintenance of items and entering, updating sales invoices.

# Transactions Module
sys-privilege-use_transactions-name = Record Transactions
sys-privilege-use_transactions-description = Permits inputting general journal entries, staging transaction drafts, and preparing data for verification.

sys-privilege-manage_transactions-name = Post & Modify Transactions
sys-privilege-manage_transactions-description = Permits finalizing journal entries, posting to the ledger, and issuing structural transaction reversals (subject to Strict Audit Mode).

# Administrative Tools
sys-privilege-manage_users-name = Manage Users & Team Roles
sys-privilege-manage_users-description = Permits inviting team members, adjusting individual access levels, configuring dynamic roles, or deactivating accounts.

sys-privilege-manage_organization-name = Manage Organization Settings
sys-privilege-manage_organization-description = Permits altering core workspace parameters, toggling Strict Audit Mode, opening/closing accounting periods, and locking historical fiscal years.

# Branding
branding-app-name = KelpieBooks
branding-app-subtitle = SME Accounting Engine

# Dashboard
dashboard-title = Dashboard
dashboard-period-locked = 🔒 Period Locked Until: { $date }
dashboard-period-open = 🔓 Period Open
dashboard-net-profit-ytd = Net Profit (YTD)
dashboard-operating-bank = Operating Bank
dashboard-receivables = Receivables
dashboard-payables = Payables
dashboard-recent-ledger-activity = Recent Ledger Activity
dashboard-top-5-payables = Top 5 Payables

# Common terms
common-date = Date
common-description = Description
common-amount = Amount
common-vendor = Vendor
common-due-date = Due Date
common-loading = Loading...
common-toggle = Toggle
common-code = Code
common-name = Name
common-category = Category
common-balance = Balance
common-actions = Actions
common-cancel = Cancel
common-edit = Edit
common-delete = Delete
common-expand = Expand
common-collapse = Collapse
common-confirm-deletion = Confirm Deletion
common-confirm-delete-button = Confirm Delete
common-deletion-confirm-warning = This action cannot be undone.
common-debit = Debit
common-credit = Credit
common-account = Account
common-network-error = Network error: { $error }
common-total = Total
common-none = None
common-list = List
common-aged = Aged
common-net = Net
common-tax = Tax
common-tax-rate = Tax rate
common-gross = Gross
common-pay = Pay
common-view = View
common-customer = Customer
common-type = Type
common-close = Close
common-general = General
common-addresses = Addresses
common-contacts = Contacts
common-primary = Primary
common-saved = Saved!
common-items = Items
common-item = Item
common-payments = Payments
common-save = Save
common-toggle-password-visibility = Toggle password visibility
common-more = More+
common-present = Present
common-quantity = Quantity
common-price = Price

# Login Page
login-help-text = Need help? Contact your administrator.
login-form-email-label = User Email:
login-form-password-label = Password:
login-form-submit-button = Sign In
login-logo-alt-text = KelpieBooks Logo
login-forgot-password = Forgot Password?

# Login Error Messages
login-error-parse-response = Failed to parse login response.
login-error-failed = Login failed: { $status }

# Forgot Password Page
forgot-password-subtitle = Reset your password
forgot-password-success-message = If an account with that email exists, a password reset link has been sent.
forgot-password-back-to-login = Back to Login
forgot-password-email-label = Email:
forgot-password-submit-button = Send Reset Link

# Reset Password Page
reset-password-subtitle = Reset Your Password
reset-password-success-message = Your password has been reset successfully.
reset-password-back-to-login = Back to Login
reset-password-new-password-label = New Password:
reset-password-confirm-password-label = Confirm New Password:
reset-password-submit-button = Reset Password
reset-password-error-server = Error resetting password: { $status }

# Email
email-reset-subject = KelpieBooks - Secure Password Reset Request
email-reset-body-plain =
    Hello,
        A password reset request was made for your KelpieBooks account. Please visit the following link within 20 minutes to initialize a new password:
        { $reset_link }
email-reset-body-html =
    <h3>KelpieBooks Security Authentication</h3>
    <p>A password reset request was initiated for your profile access.</p>
    <p><a href="{ $reset_link }" style="display:inline-block; padding:10px 20px; background:#2563eb; color:white; text-decoration:none; border-radius:5px;">Reset My Password</a></p>
    <p><small>If you did not make this request, you can safely ignore this correspondence.</small></p>

# Sidebar
sidebar-logo-alt = Logo
sidebar-dashboard = Dashboard
sidebar-accounts = Accounts
sidebar-payables = Payables
sidebar-partners = Partners
sidebar-reports = Reports
sidebar-trial-balance = Trial Balance
sidebar-profit-loss = Profit & Loss
sidebar-balance-sheet = Balance Sheet
sidebar-general-ledger = General Ledger
sidebar-aged-payables = Aged Payables
sidebar-tasks = Tasks
sidebar-close-year = Close Year
sidebar-period-settings = Period Settings
sidebar-configuration = Configuration
sidebar-admin = Admin
sidebar-users = Users
sidebar-roles = Roles
sidebar-sales = Sales

# Sales invoice list
sales-invoice-list = Sales Invoices

# Header
header-toggle-menu-alt = Toggle menu
header-profile-alt = Profile
header-edit-profile = Edit Profile
header-logout-alt = Logout
header-logout = Logout

# Chart of Accounts
coa-title = Chart of Accounts
coa-description = This is a list of all accounts in your organization. The balances include all transactions and are rolled up into parent accounts.
coa-add-account-button = Add Account

# Chart of Accounts Error Messages
coa-error-parse-accounts = Failed to parse accounts: { $error }
coa-error-fetch-accounts = Failed to fetch accounts: { $status }
coa-error-add-account = Failed to add account: { $status }
coa-error-update-account = Failed to update account: { $status }
coa-error-not-found = Account not found
coa-error-delete-account = Failed to delete account: { $status }

# Add/Edit Account Modal
account-modal-add-title = Add New Account
account-modal-edit-title = Edit Account
account-modal-code-label = Code:
account-modal-name-label = Name:
account-modal-category-label = Category:
account-modal-parent-label = Parent Account:
account-modal-parent-none = None (Root Account)
account-modal-is-group-label = Is Group:
account-modal-is-bank-account-label = Is Bank Account:
account-modal-save-button = Save Changes
account-modal-add-button = Add Account

# Account Categories
account-category-asset = Asset
account-category-liability = Liability
account-category-equity = Equity
account-category-revenue = Revenue
account-category-expense = Expense

# Delete Confirmation Modal
account-delete-confirm-message = Are you sure you want to delete the account: { $name }?
account-delete-confirm-warning = This action cannot be undone. You can only delete accounts with no transactions.

# Account Ledger
ledger-title = Ledger: { $name }
ledger-add-transaction-button = Add New Transaction
ledger-opening-balance = Opening Balance

# Account Ledger Error Messages
ledger-error-parse-entries = Failed to parse entries: { $error }
ledger-error-fetch-entries = Failed to fetch entries: { $status }
ledger-error-reverse-transaction = Failed to reverse transaction: { $status }
ledger-error-delete-transaction = Failed to delete transaction: { $status }

# Journal Entry Row
journal-entry-select-account = Select Account
journal-entry-description-placeholder = Description
journal-entry-currency-placeholder = 0.00

# Transaction Row
transaction-row-reverse = Reverse
transaction-row-duplicate = Duplicate
transaction-row-loading-details = Loading details...
transaction-row-details-for = Details for transaction
transaction-row-error-load-details = Could not load transaction details.

# Reversal Confirmation Modal
reversal-confirm-title = Confirm Transaction Reversal
reversal-confirm-original-description = Original Description:
reversal-confirm-reversal-description = Reversal Description
reversal-confirm-warning = This action cannot be undone.
reversal-confirm-button = Confirm Reversal

# Transaction Error Messages
transaction-error-parse = Failed to parse transaction: { $error }
transaction-error-fetch = Failed to fetch transaction: { $status }

# Profile Page
profile-title = Edit Profile
profile-details-title = Your Details
profile-email-label = Email:
profile-full-name-label = Full Name:
profile-display-name-label = Display Name:
profile-save-details-button = Save Details
profile-save-success-message = Profile saved successfully!
profile-change-password-title = Change Password
profile-old-password-label = Old Password:
profile-new-password-label = New Password:
profile-confirm-password-label = Confirm New Password:
profile-change-password-button = Change Password
profile-password-change-success = Password changed successfully!

# Profile Page Error Messages
profile-error-parse-response = Failed to parse server response.
profile-error-save-profile = Error saving profile: { $status }
profile-error-change-password = Error changing password: { $status }

# Register Page
register-title = Create your Account
register-org-name-label = Organization Name:
register-full-name-label = Full Name:
register-display-name-label = Display Name (Optional):
register-email-label = Email:
register-password-label = Password:
register-coa-template-label = Chart of Accounts Template:
register-submit-button = Register
register-create-org-subtitle = Create a new organization cluster
register-help-text = Looking to join an existing team? Contact your system administrator for access.
register-back-to-login = Back to Login

# Register Page Error Messages
register-error-server = Server error: { $status }

# Close Year Page
close-year-title = Close Financial Year
close-year-description = Closing the financial year is an irreversible process. It will summarize all revenue and expense accounts into Retained Earnings and lock all transactions on or before the selected date.
close-year-select-date-label = Select Year-End Date
close-year-button = Close Financial Year
close-year-loading-message = Closing year...
close-year-confirm-title = Confirm Year-End Close
close-year-confirm-message = Are you sure you want to close the financial year ending on { $date }? This action cannot be undone.
close-year-confirm-button = Yes, Close Year
close-year-success-message = Financial year closed successfully.

# Close Year Page Error Messages
close-year-error = Error { $status }: { $error }

# Profit & Loss Page
profit-loss-title = Profit & Loss
profit-loss-revenue-section = Revenue
profit-loss-expenses-section = Expenses
profit-loss-net-income = Net Income

# Profit & Loss Page Error Messages
profit-loss-error-parse = Failed to parse P&L data: { $error }
profit-loss-error-fetch = Error fetching P&L: { $status }

# Balance Sheet Page
balance-sheet-title = Balance Sheet
balance-sheet-assets-section = Assets
balance-sheet-total-assets = Total Assets
balance-sheet-liabilities-section = Liabilities
balance-sheet-total-liabilities = Total Liabilities
balance-sheet-equity-section = Equity
balance-sheet-current-year-earnings = Current Year Earnings
balance-sheet-total-equity = Total Equity
balance-sheet-total-liabilities-equity = Total Liabilities & Equity

# Balance Sheet Page Error Messages
balance-sheet-error-parse = Failed to parse Balance Sheet data: { $error }
balance-sheet-error-fetch = Error fetching Balance Sheet: { $status }

# Configuration Page
configuration-title = Configuration
configuration-org-settings-title = Organization Settings
configuration-strict-audit-label = Strict Audit Mode
configuration-strict-audit-description = When enabled, forbids editing and deletion of Journal Entries for closed periods.
configuration-system-accounts-title = System Accounts
configuration-system-accounts-description = Map system-critical accounts to the correct accounts in your chart of accounts.
configuration-select-account = Select Account
configuration-save-button = Save Configuration
configuration-save-success = Configuration saved successfully!

# Configuration Page Error Messages
configuration-error-parse = Failed to parse data
configuration-error-fetch = Failed to fetch data
configuration-error-save = Error saving configuration: { $status }

# Trial Balance Page
trial-balance-title = Trial Balance

# Trial Balance Page Error Messages
trial-balance-error-parse = Failed to parse Trial Balance data: { $error }
trial-balance-error-fetch = Error fetching Trial Balance: { $status }

# General Ledger Report Page
general-ledger-title = General Ledger Detail

# General Ledger Report Page Error Messages
general-ledger-error-parse = Failed to parse report data: { $error }
general-ledger-error-fetch = Error fetching report: { $status }

# New Transaction Page
new-transaction-edit-title = Edit Journal Transaction
new-transaction-new-title = New Journal Transaction
new-transaction-update-button = Update Transaction
new-transaction-save-button = Save Transaction
new-transaction-for-label = For:
new-transaction-date-label = Date:
new-transaction-add-line-button = Add Line
new-transaction-debits-total = Debits: { $amount }
new-transaction-credits-total = Credits: { $amount }
new-transaction-balanced = Balanced
new-transaction-unbalanced = Unbalanced
new-transaction-period-locked = Period Locked

# Period Settings Page
period-settings-title = Accounting Period Settings
period-settings-description = Prevent changes to transactions on or before this date:
period-settings-update-button = Update Lock Date
period-settings-current-lock = Current Lock:

# Payables Ledger Page
payables-ledger-title = Payables Ledger
payables-ledger-new-invoice-button = + New Invoice

# Sales Ledger Page
sales-ledger-title = Sales Ledger
sales-ledger-new-invoice-button = + New Invoice

# Aged Trial Balance Matrix
aged-trial-balance-current = Current
aged-trial-balance-1-30-days = 1-30 Days
aged-trial-balance-31-60-days = 31-60 Days
aged-trial-balance-61-90-days = 61-90 Days
aged-trial-balance-90-plus-days = 90+ Days

# Aged Trial Balance Matrix Error Messages
aged-trial-balance-error-parse = Failed to parse summary: { $error }
aged-trial-balance-error-fetch = Failed to fetch summary: { $status }

# Vendor Invoice Filter
vendor-invoice-filter-outstanding = Outstanding
vendor-invoice-filter-fully-paid = Fully Paid
vendor-invoice-filter-all-invoices = All Invoices
vendor-invoice-filter-from-label = From:
vendor-invoice-filter-to-label = To:
vendor-invoice-filter-vendor-label = Vendor:
vendor-invoice-filter-all-vendors = All Vendors
vendor-invoice-filter-min-amount-label = Min Amount:

# Sales Invoice Filter
sales-invoice-filter-draft = Draft
sales-invoice-filter-outstanding = Outstanding
sales-invoice-filter-fully-paid = Fully Paid
sales-invoice-filter-all-invoices = All Invoices
sales-invoice-filter-from-label = From:
sales-invoice-filter-to-label = To:
sales-invoice-filter-customer-label = Customer:
sales-invoice-filter-all-customers = All Customers
sales-invoice-filter-min-amount-label = Min Amount:

# Vendor Invoice Table
vendor-invoice-table-invoice-number = Invoice #
vendor-invoice-table-invoice-date = Invoice Date
vendor-invoice-table-balance-due = Balance Due

# Sales Invoice Table
sales-invoice-table-invoice-number = Invoice #
sales-invoice-table-invoice-date = Invoice Date

# Vendor Invoice Table Error Messages
vendor-invoice-table-error-parse-invoices = Failed to parse invoices: { $error }
vendor-invoice-table-error-fetch-invoices = Failed to fetch invoices: { $status }
vendor-invoice-table-error-parse-partner = Failed to parse partner: { $error }
vendor-invoice-table-error-fetch-partner = Failed to fetch partner: { $status }
vendor-invoice-table-error-parse-invoice = Failed to parse invoice: { $error }
vendor-invoice-table-error-fetch-invoice = Failed to fetch invoice: { $status }

# Sales Invoice Table Error Messages
sales-invoice-table-error-parse-invoices = Failed to parse invoices: { $error }
sales-invoice-table-error-fetch-invoices = Failed to fetch invoices: { $status }
sales-invoice-table-error-parse-partner = Failed to parse partner: { $error }
sales-invoice-table-error-fetch-partner = Failed to fetch partner: { $status }
sales-invoice-table-error-parse-invoice = Failed to parse invoice: { $error }
sales-invoice-table-error-fetch-invoice = Failed to fetch invoice: { $status }

# New Vendor Invoice Page
new-vendor-invoice-title = New Vendor Invoice
new-vendor-invoice-select-vendor = Select a vendor
new-vendor-invoice-number-label = Invoice Number:
new-vendor-invoice-date-label = Invoice Date:
new-vendor-invoice-due-date-label = Due Date:
new-vendor-invoice-net-amount = Net Amount
new-vendor-invoice-tax-amount = Tax Amount
new-vendor-invoice-add-line-button = + Add Line
new-vendor-invoice-save-button = Save Invoice

# New Vendor Invoice Page Error Messages
new-vendor-invoice-error-parse-vendors = Failed to parse vendors: { $error }
new-vendor-invoice-error-fetch-vendors = Failed to fetch vendors: { $status }
new-vendor-invoice-error-parse-accounts = Failed to parse accounts: { $error }
new-vendor-invoice-error-fetch-accounts = Failed to fetch accounts: { $status }
new-vendor-invoice-error-create-invoice = Failed to create invoice: { $status }

# Partner List Page
partner-list-title = Partners
partner-list-description = This is a list of all partners in your organization.
partner-list-add-partner-button = Add Partner
partner-list-legal-name = Legal Name
partner-list-trade-name = Trade Name

# Partner List Page Error Messages
partner-list-error-parse-partners = Failed to parse partners: { $error }
partner-list-error-fetch-partners = Failed to fetch partners: { $status }
partner-list-error-parse-accounts = Failed to parse accounts: { $error }
partner-list-error-fetch-accounts = Failed to fetch accounts: { $status }
partner-list-error-parse-partner = Failed to parse partner: { $error }
partner-list-error-fetch-partner = Failed to fetch partner: { $status }
partner-list-error-parse-addresses = Failed to parse addresses: { $error }
partner-list-error-fetch-addresses = Failed to fetch addresses: { $status }
partner-list-error-parse-contacts = Failed to parse contacts: { $error }
partner-list-error-fetch-contacts = Failed to fetch contacts: { $status }
partner-list-error-add-partner = Failed to add partner: { $status }
partner-list-error-delete-partner = Failed to delete partner: { $status }

# Add Partner Modal
add-partner-title = Add New Partner
add-partner-legal-name-label = Legal Name:
add-partner-trade-name-label = Trade Name:
add-partner-tax-identifier-label = Tax Identifier:
add-partner-is-vendor-label = Is Vendor:
add-partner-is-customer-label = Is Customer:
add-partner-default-ap-account-label = Default AP Account:
add-partner-default-ar-account-label = Default AR Account:

# Delete Partner Confirmation Modal
delete-partner-confirm-message = Are you sure you want to delete the partner: { $name }?

# Partner Row
partner-row-vendor-customer = Vendor & Customer

# Report Options
report-options-from-label = From:
report-options-to-label = To:
report-options-export-csv-tooltip = Export to CSV
report-options-export-pdf-tooltip = Export to PDF
report-options-accounts-label = Accounts:
report-options-min-amount-label = Min Amount:
report-options-all-accounts = All Accounts
report-options-selected-accounts = { $count } Selected

# Partner Drawer
partner-drawer-error-save = Failed to save partner: { $status }

# Address Edit Card
address-edit-card-edit-title = Edit Address
address-edit-card-add-title = Add Address
address-edit-card-line1-label = Addr line 1:
address-edit-card-line1-placeholder = Address Line 1
address-edit-card-line2-label = Addr line 2:
address-edit-card-line2-placeholder = Address Line 2 (Optional)
address-edit-card-city-label = City:
address-edit-card-city-placeholder = City
address-edit-card-state-label = State:
address-edit-card-state-placeholder = State
address-edit-card-post-code-label = Post Code:
address-edit-card-post-code-placeholder = Postcode
address-edit-card-country-label = Country:
address-edit-card-country-placeholder = Country
address-edit-card-address-type-label = Address Type:
address-edit-card-save-button = Save Address

# Addresses View
addresses-view-add-button = Add Address
addresses-view-error-save = Failed to save address: { $status }
addresses-view-error-delete = Failed to delete address: { $status }

# Contact Edit Card
contact-edit-card-edit-title = Edit Contact
contact-edit-card-add-title = Add Contact
contact-edit-card-full-name-label = Full Name
contact-edit-card-preferred-name-label = Preferred Name
contact-edit-card-email-label = Email address
contact-edit-card-email-placeholder = Email
contact-edit-card-phone-label = Phone number
contact-edit-card-phone-placeholder = Phone
contact-edit-card-role-title-label = Role/Title
contact-edit-card-save-button = Save Contact

# Contacts View
contacts-view-add-button = Add Contact
contacts-view-no-role = No role specified
contacts-view-error-save = Failed to save contact: { $status }
contacts-view-error-delete = Failed to delete contact: { $status }

# Delete Address Confirmation Modal
delete-address-confirm-message = Are you sure you want to delete the address: { $address }?

# Delete Contact Confirmation Modal
delete-contact-confirm-message = Are you sure you want to delete the contact: { $name } { $preferred_name }?

# Vendor Invoice Drawer
vendor-invoice-drawer-inv-number = Inv #: { $number }
vendor-invoice-drawer-gross = Gross: { $amount }
vendor-invoice-drawer-outstanding-balance = Outstanding Balance: { $amount }

# Sales Invoice Drawer
sales-invoice-drawer-inv-number = Inv #: { $number }

# Details View
details-view-error-update = Failed to update invoice: { $status }
details-view-notes-label = Notes:

# Items View
items-view-unknown-gl-account = Unknown GL Account
items-view-gl-label = GL: { $account }
items-view-net-tax-breakdown = Net: { $net } | Tax: { $tax }
items-view-add-item-button = + Add Item
items-view-delete-item-title = Delete Item
items-view-delete-item-message = Are you sure you want to delete the item: { $description }?
items-view-error-update-items = Failed to update invoice items: { $status }

# Payments View
payments-view-payment-date-label = Payment Date:
payments-view-bank-account-label = Bank Account:
payments-view-reference-label = Reference:
payments-view-make-payment-button = Make Payment
payments-view-error-parse-payments = Failed to parse payments: { $error }
payments-view-error-fetch-payments = Failed to fetch payments: { $status }
payments-view-error-parse-accounts = Failed to parse accounts: { $error }
payments-view-error-fetch-accounts = Failed to fetch accounts: { $status }
payments-view-error-make-payment = Failed to make payment: { $status }

# Item Edit Card
item-edit-card-add-title = Add Item
item-edit-card-edit-title = Edit Item
item-edit-card-net-amount-label = Net Amount:
item-edit-card-tax-amount-label = Tax Amount:

# Account Ledger Export
account-ledger-export-report-qualifier = Account { $account_name } for Period { $start_date } - { $end_date }
account-ledger-export-title = Journal Entries

# Balance Sheet Export
balance-sheet-export-assets-header = Assets,
balance-sheet-export-total-assets = Total Assets
balance-sheet-export-liabilities-header = Liabilities,
balance-sheet-export-total-liabilities = Total Liabilities
balance-sheet-export-equity-header = Equity,
balance-sheet-export-current-year-earnings = Current Year Earnings
balance-sheet-export-total-equity = Total Equity
balance-sheet-export-total-liabilities-equity = Total Liabilities & Equity
balance-sheet-export-as-at = As at { $date }

# General Ledger Export
general-ledger-export-period = Period { $start_date } - { $end_date }

# Profit Loss Export
profit-loss-export-revenue-header = Revenue,
profit-loss-export-expenses-header = Expenses,

# Trial Balance Export
trial-balance-export-total = Total

# Users Page
users-title = Users
users-list-description = This is a list of all the users in your organization.
users-add-button = Add User
users-header-email = Email
users-header-full-name = Full Name
users-header-display-name = Display Name
users-header-role = Role
users-error-parse = Failed to parse users: { $error }
users-error-fetch = Failed to fetch users: { $status }
users-error-delete = Failed to delete user: { $error }
users-error-add = Failed to add user: { $error }
users-error-update = Failed to update user: { $error }

# User Modals
user-modal-add-title = Add New User
user-modal-edit-title = Edit User
user-modal-email-label = Email:
user-modal-full-name-label = Full Name:
user-modal-display-name-label = Display Name:
user-modal-password-label = Password:
user-modal-role-label = Role:
user-modal-select-role = Select a role
user-modal-add-button = Add User
user-modal-save-button = Save Changes
delete-user-confirm-title = Delete User
delete-user-confirm-message = Are you sure you want to delete the user: { $user }?

# Roles Page
roles-title = Roles
roles-list-description = This is a list of all the roles in your organization.
roles-add-button = Add Role
roles-header-name = Name
roles-error-parse = Failed to parse roles: { $error }
roles-error-fetch = Failed to fetch roles: { $status }
roles-error-delete = Failed to delete role: { $error }
roles-error-add = Failed to add role: { $error }
roles-error-update = Failed to update role: { $error }

# Role Modals
role-modal-add-title = Add New Role
role-modal-edit-title = Edit Role
role-modal-name-label = Name:
role-modal-privileges-label = Privileges:
role-modal-add-button = Add Role
role-modal-save-button = Save Changes
delete-role-confirm-title = Delete Role
delete-role-confirm-message = Are you sure you want to delete the role: { $role }?

#Sales
new-sales-invoice-title = New Sales Invoice
new-sales-invoice-number-label = Invoice number
new-sales-invoice-date-label = Date
new-sales-invoice-due-date-label = Due Date
new-sales-invoice-select-customer = Select Item
new-sales-invoice-select-item = Select Item
new-sales-invoice-add-line-button = + Add line
new-sales-invoice-error-parse-items = Error parsing items
new-sales-invoice-save-button = Save
new-sales-invoice-success = Sales invoice { $number } was created successfully.
new-sales-invoice-error-parse-response = Failed to parse created invoice: { $error }

# Sales Invoice — Addresses
new-sales-invoice-billing-address = Billing address
new-sales-invoice-select-billing = Select billing address
new-sales-invoice-billing-override = Billing address override
new-sales-invoice-shipping-address = Shipping address
new-sales-invoice-select-shipping = Select shipping address
new-sales-invoice-shipping-override = Shipping address override

# Address field labels/placeholders
address-name = Name
address-attention = Attention
address-line1 = Address line 1
address-line2 = Address line 2
address-city = City
address-region = State/Province/Region
address-postal-code = Postal code
address-country = Country

# Sales items
item-list-title = Items
item-list-description = This is a list of all the items in your organization.
item-list-code = Code
item-list-name = Name
item-list-type = Type
item-list-price = Price
item-list-add-item-button = Add Item
item-list-error-parse-items = Failed to parse items: { $error }
item-list-error-fetch-items = Failed to fetch items: { $status }
item-edit-title = Edit Item
item-add-title = Add Item
item-code-label = Code:
item-name-label = Name:
item-description-label = Description:
item-type-label = Type:
item-uom-label = Unit of Measure:
item-select-uom = Select a unit of measure
item-price-label = Price:
item-tax-category-label = Tax Category:
item-select-tax-category = Select a tax category
item-income-account-label = Income Account:
item-select-income-account = Select an income account
item-is-active-label = Is Active:
item-filter-search-placeholder = Search by code or name
item-filter-all-types = All Types
item-type-service = Service
item-type-stocked = Stocked
item-type-non-stocked = Non-Stocked
item-filter-include-inactive = Include Inactive
uom-list-title = Units of Measure
uom-list-description = This is a list of all the units of measure in your organization.
uom-list-add-uom-button = Add Unit of Measure
uom-list-code = Code
uom-list-name = Name
uom-list-is-active = Is Active
uom-list-error-parse-uoms = Failed to parse units of measure: { $error }
uom-list-error-fetch-uoms = Failed to fetch units of measure: { $status }
uom-add-title = Add Unit of Measure
uom-edit-title = Edit Unit of Measure
uom-code-label = Code:
uom-name-label = Name:
uom-is-active-label = Is Active:
uom-delete-title = Delete Unit of Measure
uom-delete-confirm-message = Are you sure you want to delete the unit of measure: { $name }?
uom-delete-error = Failed to delete unit of measure. It may be in use.
tax-category-list-title = Tax Categories
tax-category-list-description = This is a list of all the tax categories in your organization.
tax-category-list-add-tax-category-button = Add Tax Category
tax-category-list-name = Name
tax-category-list-is-active = Is Active
tax-category-list-error-parse-tax-categories = Failed to parse tax categories: { $error }
tax-category-list-error-fetch-tax-categories = Failed to fetch tax categories: { $status }
tax-category-add-title = Add Tax Category
tax-category-edit-title = Edit Tax Category
tax-category-name-label = Name:
tax-category-description-label = Description:
tax-category-is-active-label = Is Active:
tax-category-delete-title = Delete Tax Category
tax-category-delete-confirm-message = Are you sure you want to delete the tax category: { $name }?
tax-category-delete-error = Failed to delete tax category. It may be in use.
tax-category-row-manage-rates = Manage Rates

# Tax Rate Drawer
tax-rate-drawer-error-parse-rates = Failed to parse tax rates: { $error }
tax-rate-drawer-error-fetch-rates = Failed to fetch tax rates: { $status }
tax-rate-drawer-error-update-rates = Failed to update tax rates: { $status }
tax-rate-drawer-add-rate-button = + Add Rate
tax-rate-drawer-validity = Valid from { $from } to { $to }
tax-rate-drawer-delete-rate-title = Delete Rate
tax-rate-drawer-delete-rate-message = Are you sure you want to delete the rate: { $name }?

# Tax Rate Edit Card
tax-rate-edit-card-add-title = Add Rate
tax-rate-edit-card-edit-title = Edit Rate
tax-rate-edit-card-rate-label = Rate:
tax-rate-edit-card-valid-from-label = Valid From:
tax-rate-edit-card-valid-to-label = Valid To:

# Security
security-error-no-admin = You cannot perform this action. At least one security administrator must remain.

# Test keys
test-key = Test Value
test-key-override = Test Value 2
test-key-args = Hello, { $name }!
