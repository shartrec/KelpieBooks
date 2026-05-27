##
## Fichier de traduction Fluent pour KelpieBooks (fr)
##

# Branding
branding-app-name = KelpieBooks
branding-app-subtitle = Moteur comptable PME

# Dashboard
dashboard-title = Tableau de bord
dashboard-period-locked = 🔒 Période verrouillée jusqu’au : { $date }
dashboard-period-open = 🔓 Période ouverte
dashboard-net-profit-ytd = Bénéfice net (cumul annuel)
dashboard-operating-bank = Banque d’exploitation
dashboard-receivables = Créances clients
dashboard-payables = Dettes fournisseurs
dashboard-recent-ledger-activity = Activité récente du grand livre
dashboard-top-5-payables = Top 5 des dettes fournisseurs

# Common terms
common-date = Date
common-description = Description
common-amount = Montant
common-vendor = Fournisseur

common-due-date = Date d’échéance
common-loading = Chargement...
common-toggle = Basculer
common-code = Code
common-name = Nom
common-category = Catégorie
common-balance = Solde
common-actions = Actions
common-cancel = Annuler
common-edit = Modifier
common-delete = Supprimer
common-expand = Développer
common-collapse = Réduire
common-confirm-deletion = Confirmer la suppression
common-confirm-delete-button = Confirmer la suppression
common-debit = Débit
common-credit = Crédit
common-account = Compte
common-network-error = Erreur réseau : { $error }
common-total = Total
common-none = Aucun
common-list = Liste
common-aged = Ancienneté
common-net = Net
common-tax = Taxe
common-gross = Brut
common-pay = Payer
common-view = Voir
common-customer = Client
common-type = Type
common-close = Fermer
common-general = Général
common-addresses = Adresses
common-contacts = Contacts
common-primary = Principal
common-saved = Enregistré !
common-items = Articles
common-payments = Paiements
common-save = Enregistrer

# Login Page
login-help-text = Besoin d’aide ? Contactez votre administrateur.
login-form-email-label = E-mail utilisateur :
login-form-password-label = Mot de passe :
login-form-submit-button = Se connecter
login-logo-alt-text = Logo KelpieBooks

# Login Error Messages
login-error-parse-response = Impossible d’analyser la réponse de connexion.
login-error-failed = Échec de connexion : { $status }

# Sidebar
sidebar-logo-alt = Logo
sidebar-dashboard = Tableau de bord
sidebar-accounts = Comptes
sidebar-payables = Dettes fournisseurs
sidebar-partners = Partenaires
sidebar-reports = Rapports
sidebar-trial-balance = Balance de vérification
sidebar-profit-loss = Compte de résultat
sidebar-balance-sheet = Bilan
sidebar-general-ledger = Grand livre
sidebar-tasks = Tâches
sidebar-close-year = Clôturer l’exercice
sidebar-period-settings = Paramètres de période
sidebar-configuration = Configuration

# Header
header-toggle-menu-alt = Basculer le menu
header-profile-alt = Profil
header-edit-profile = Modifier le profil
header-logout-alt = Déconnexion
header-logout = Déconnexion

# Chart of Accounts
coa-title = Plan comptable
coa-description = Voici la liste de tous les comptes de votre organisation. Les soldes incluent toutes les transactions et sont regroupés dans les comptes parents.
coa-add-account-button = Ajouter un compte

# Chart of Accounts Error Messages
coa-error-parse-accounts = Impossible d’analyser les comptes : { $error }
coa-error-fetch-accounts = Impossible de récupérer les comptes : { $status }
coa-error-add-account = Impossible d’ajouter le compte : { $status }
coa-error-update-account = Impossible de mettre à jour le compte : { $status }
coa-error-delete-account = Impossible de supprimer le compte : { $status }

# Add/Edit Account Modal
account-modal-add-title = Ajouter un nouveau compte
account-modal-edit-title = Modifier le compte
account-modal-code-label = Code :
account-modal-name-label = Nom :
account-modal-category-label = Catégorie :
account-modal-parent-label = Compte parent :
account-modal-parent-none = Aucun (compte racine)
account-modal-is-group-label = Est un groupe :
account-modal-is-bank-account-label = Est un compte bancaire :
account-modal-save-button = Enregistrer les modifications
account-modal-add-button = Ajouter le compte

# Account Categories
account-category-asset = Actif
account-category-liability = Passif
account-category-equity = Capitaux propres
account-category-revenue = Produits
account-category-expense = Charges

# Delete Confirmation Modal
delete-confirm-message = Êtes-vous sûr de vouloir supprimer le compte : { $name } ?
delete-confirm-warning = Cette action est irréversible. Vous ne pouvez supprimer que les comptes sans transactions.

# Account Ledger
ledger-title = Grand livre : { $name }
ledger-add-transaction-button = Ajouter une nouvelle transaction
ledger-opening-balance = Solde d’ouverture

# Account Ledger Error Messages
ledger-error-parse-entries = Impossible d’analyser les écritures : { $error }
ledger-error-fetch-entries = Impossible de récupérer les écritures : { $status }
ledger-error-reverse-transaction = Impossible d’annuler la transaction : { $status }
ledger-error-delete-transaction = Impossible de supprimer la transaction : { $status }

# Journal Entry Row
journal-entry-select-account = Sélectionner un compte
journal-entry-description-placeholder = Description
journal-entry-currency-placeholder = 0.00

# Transaction Row
transaction-row-reverse = Contrepasser
transaction-row-duplicate = Dupliquer
transaction-row-loading-details = Chargement des détails...
transaction-row-details-for = Détails de la transaction
transaction-row-error-load-details = Impossible de charger les détails de la transaction.

# Reversal Confirmation Modal
reversal-confirm-title = Confirmer la contrepassation de la transaction
reversal-confirm-original-description = Description d’origine :
reversal-confirm-reversal-description = Description de contrepassation
reversal-confirm-warning = Cette action est irréversible.
reversal-confirm-button = Confirmer la contrepassation

# Deletion Confirmation Modal
deletion-confirm-title = Confirmer la suppression de la transaction
deletion-confirm-warning = Cette action est irréversible. La suppression de cette transaction l’effacera définitivement de vos registres.

# Transaction Error Messages
transaction-error-parse = Impossible d’analyser la transaction : { $error }
transaction-error-fetch = Impossible de récupérer la transaction : { $status }

# Profile Page
profile-title = Modifier le profil
profile-details-title = Vos informations
profile-email-label = E-mail :
profile-full-name-label = Nom complet :
profile-display-name-label = Nom d’affichage :
profile-save-details-button = Enregistrer les informations
profile-save-success-message = Profil enregistré avec succès !
profile-change-password-title = Modifier le mot de passe
profile-old-password-label = Ancien mot de passe :
profile-new-password-label = Nouveau mot de passe :
profile-confirm-password-label = Confirmer le nouveau mot de passe :
profile-change-password-button = Modifier le mot de passe
profile-password-change-success = Mot de passe modifié avec succès !

# Profile Page Error Messages
profile-error-parse-response = Impossible d’analyser la réponse du serveur.
profile-error-save-profile = Erreur lors de l’enregistrement du profil : { $status }
profile-error-change-password = Erreur lors du changement de mot de passe : { $status }

# Register Page
register-title = Créez votre compte
register-org-name-label = Nom de l’organisation :
register-full-name-label = Nom complet :
register-display-name-label = Nom d’affichage (optionnel) :
register-email-label = E-mail :
register-password-label = Mot de passe :
register-coa-template-label = Modèle de plan comptable :
register-submit-button = S’inscrire

# Register Page Error Messages
register-error-server = Erreur serveur : { $status }

# Close Year Page
close-year-title = Clôturer l’exercice comptable
close-year-description = La clôture de l’exercice est un processus irréversible. Elle regroupera tous les comptes de produits et charges dans les bénéfices non distribués et verrouillera toutes les transactions à la date sélectionnée ou avant.
close-year-select-date-label = Sélectionner la date de clôture
close-year-button = Clôturer l’exercice
close-year-loading-message = Clôture de l’exercice...
close-year-confirm-title = Confirmer la clôture annuelle
close-year-confirm-message = Êtes-vous sûr de vouloir clôturer l’exercice se terminant le { $date } ? Cette action est irréversible.
close-year-confirm-button = Oui, clôturer l’exercice
close-year-success-message = Exercice clôturé avec succès.

# Close Year Page Error Messages
close-year-error = Erreur { $status } : { $error }

# Profit & Loss Page
profit-loss-title = Compte de résultat
profit-loss-revenue-section = Produits
profit-loss-expenses-section = Charges
profit-loss-net-income = Résultat net

# Profit & Loss Page Error Messages
profit-loss-error-parse = Impossible d’analyser les données du compte de résultat : { $error }
profit-loss-error-fetch = Erreur lors de la récupération du compte de résultat : { $status }

# Balance Sheet Page
balance-sheet-title = Bilan
balance-sheet-assets-section = Actifs
balance-sheet-total-assets = Total des actifs
balance-sheet-liabilities-section = Passifs
balance-sheet-total-liabilities = Total des passifs
balance-sheet-equity-section = Capitaux propres
balance-sheet-current-year-earnings = Résultat de l’exercice en cours
balance-sheet-total-equity = Total des capitaux propres
balance-sheet-total-liabilities-equity = Total passifs et capitaux propres

# Balance Sheet Page Error Messages
balance-sheet-error-parse = Impossible d’analyser les données du bilan : { $error }
balance-sheet-error-fetch = Erreur lors de la récupération du bilan : { $status }

# Configuration Page
configuration-title = Configuration
configuration-org-settings-title = Paramètres de l’organisation
configuration-strict-audit-label = Mode audit strict
configuration-strict-audit-description = Lorsqu’il est activé, interdit la modification et la suppression des écritures comptables pour les périodes clôturées.
configuration-system-accounts-title = Comptes système
configuration-system-accounts-description = Associez les comptes critiques du système aux comptes appropriés dans votre plan comptable.
configuration-select-account = Sélectionner un compte
configuration-save-button = Enregistrer la configuration
configuration-save-success = Configuration enregistrée avec succès !

# Configuration Page Error Messages
configuration-error-parse = Impossible d’analyser les données
configuration-error-fetch = Impossible de récupérer les données
configuration-error-save = Erreur lors de l’enregistrement de la configuration : { $status }

# Trial Balance Page
trial-balance-title = Balance de vérification

# Trial Balance Page Error Messages
trial-balance-error-parse = Impossible d’analyser les données de la balance de vérification : { $error }
trial-balance-error-fetch = Erreur lors de la récupération de la balance de vérification : { $status }

# General Ledger Report Page
general-ledger-title = Détail du grand livre

# General Ledger Report Page Error Messages
general-ledger-error-parse = Impossible d’analyser les données du rapport : { $error }
general-ledger-error-fetch = Erreur lors de la récupération du rapport : { $status }

# New Transaction Page
new-transaction-edit-title = Modifier l’écriture comptable
new-transaction-new-title = Nouvelle écriture comptable
new-transaction-update-button = Mettre à jour la transaction
new-transaction-save-button = Enregistrer la transaction
new-transaction-for-label = Pour :
new-transaction-date-label = Date :
new-transaction-add-line-button = Ajouter une ligne
new-transaction-debits-total = Débits : { $amount }
new-transaction-credits-total = Crédits : { $amount }
new-transaction-balanced = Équilibré
new-transaction-unbalanced = Déséquilibré
new-transaction-period-locked = Période verrouillée

# Period Settings Page
period-settings-title = Paramètres des périodes comptables
period-settings-description = Empêcher les modifications des transactions à cette date ou avant :
period-settings-update-button = Mettre à jour la date de verrouillage
period-settings-current-lock = Verrouillage actuel :

# Payables Ledger Page
payables-ledger-title = Grand livre fournisseurs
payables-ledger-new-invoice-button = + Nouvelle facture

# Aged Trial Balance Matrix
aged-trial-balance-current = Courant
aged-trial-balance-1-30-days = 1-30 jours
aged-trial-balance-31-60-days = 31-60 jours
aged-trial-balance-61-90-days = 61-90 jours
aged-trial-balance-90-plus-days = Plus de 90 jours

# Aged Trial Balance Matrix Error Messages
aged-trial-balance-error-parse = Impossible d’analyser le résumé : { $error }
aged-trial-balance-error-fetch = Impossible de récupérer le résumé : { $status }

# Vendor Invoice Filter
vendor-invoice-filter-outstanding = En attente
vendor-invoice-filter-fully-paid = Entièrement payées
vendor-invoice-filter-all-invoices = Toutes les factures
vendor-invoice-filter-from-label = De :
vendor-invoice-filter-to-label = À :
vendor-invoice-filter-vendor-label = Fournisseur :
vendor-invoice-filter-all-vendors = Tous les fournisseurs
vendor-invoice-filter-min-amount-label = Montant minimum :

# Vendor Invoice Table
vendor-invoice-table-invoice-number = Facture n°
vendor-invoice-table-invoice-date = Date de facture
vendor-invoice-table-balance-due = Solde dû

# Vendor Invoice Table Error Messages
vendor-invoice-table-error-parse-invoices = Impossible d’analyser les factures : { $error }
vendor-invoice-table-error-fetch-invoices = Impossible de récupérer les factures : { $status }
vendor-invoice-table-error-parse-partner = Impossible d’analyser le partenaire : { $error }
vendor-invoice-table-error-fetch-partner = Impossible de récupérer le partenaire : { $status }
vendor-invoice-table-error-parse-invoice = Impossible d’analyser la facture : { $error }
vendor-invoice-table-error-fetch-invoice = Impossible de récupérer la facture : { $status }

# New Vendor Invoice Page
new-vendor-invoice-title = Nouvelle facture fournisseur
new-vendor-invoice-select-vendor = Sélectionner un fournisseur
new-vendor-invoice-number-label = Numéro de facture :
new-vendor-invoice-date-label = Date de facture :
new-vendor-invoice-due-date-label = Date d’échéance :
new-vendor-invoice-net-amount = Montant net
new-vendor-invoice-tax-amount = Montant des taxes
new-vendor-invoice-add-line-button = + Ajouter une ligne
new-vendor-invoice-save-button = Enregistrer la facture

# New Vendor Invoice Page Error Messages
new-vendor-invoice-error-parse-vendors = Impossible d’analyser les fournisseurs : { $error }
new-vendor-invoice-error-fetch-vendors = Impossible de récupérer les fournisseurs : { $status }
new-vendor-invoice-error-parse-accounts = Impossible d’analyser les comptes : { $error }
new-vendor-invoice-error-fetch-accounts = Impossible de récupérer les comptes : { $status }
new-vendor-invoice-error-create-invoice = Impossible de créer la facture : { $status }

# Partner List Page
partner-list-title = Partenaires
partner-list-description = Voici la liste de tous les partenaires de votre organisation.
partner-list-add-partner-button = Ajouter un partenaire
partner-list-legal-name = Raison sociale
partner-list-trade-name = Nom commercial

# Partner List Page Error Messages
partner-list-error-parse-partners = Impossible d’analyser les partenaires : { $error }
partner-list-error-fetch-partners = Impossible de récupérer les partenaires : { $status }
partner-list-error-parse-accounts = Impossible d’analyser les comptes : { $error }
partner-list-error-fetch-accounts = Impossible de récupérer les comptes : { $status }
partner-list-error-parse-partner = Impossible d’analyser le partenaire : { $error }
partner-list-error-fetch-partner = Impossible de récupérer le partenaire : { $status }
partner-list-error-parse-addresses = Impossible d’analyser les adresses : { $error }
partner-list-error-fetch-addresses = Impossible de récupérer les adresses : { $status }
partner-list-error-parse-contacts = Impossible d’analyser les contacts : { $error }
partner-list-error-fetch-contacts = Impossible de récupérer les contacts : { $status }
partner-list-error-add-partner = Impossible d’ajouter le partenaire : { $status }
partner-list-error-delete-partner = Impossible de supprimer le partenaire : { $status }

# Add Partner Modal
add-partner-title = Ajouter un nouveau partenaire
add-partner-legal-name-label = Raison sociale :
add-partner-trade-name-label = Nom commercial :
add-partner-tax-identifier-label = Identifiant fiscal :
add-partner-is-vendor-label = Est fournisseur :
add-partner-is-customer-label = Est client :
add-partner-default-ap-account-label = Compte fournisseur par défaut :
add-partner-default-ar-account-label = Compte client par défaut :

# Delete Partner Confirmation Modal
delete-partner-confirm-message = Êtes-vous sûr de vouloir supprimer le partenaire : { $name } ?

# Partner Row
partner-row-vendor-customer = Fournisseur et client

# Report Options
report-options-from-label = De :
report-options-to-label = À :
report-options-export-csv-tooltip = Exporter en CSV
report-options-export-pdf-tooltip = Exporter en PDF
report-options-accounts-label = Comptes :
report-options-min-amount-label = Montant minimum :
report-options-all-accounts = Tous les comptes
report-options-selected-accounts = { $count } sélectionnés

# Partner Drawer
partner-drawer-error-save = Impossible d’enregistrer le partenaire : { $status }

# Address Edit Card
address-edit-card-edit-title = Modifier l’adresse
address-edit-card-add-title = Ajouter une adresse
address-edit-card-line1-label = Ligne d’adresse 1 :
address-edit-card-line1-placeholder = Ligne d’adresse 1
address-edit-card-line2-placeholder = Ligne d’adresse 2 (optionnel)
address-edit-card-city-label = Ville :
address-edit-card-city-placeholder = Ville
address-edit-card-state-label = État/Région :
address-edit-card-state-placeholder = État/Région
address-edit-card-post-code-label = Code postal :
address-edit-card-post-code-placeholder = Code postal
address-edit-card-country-label = Pays :
address-edit-card-country-placeholder = Pays
address-edit-card-save-button = Enregistrer l’adresse

# Addresses View
addresses-view-add-button = Ajouter une adresse
addresses-view-error-save = Impossible d’enregistrer l’adresse : { $status }
addresses-view-error-delete = Impossible de supprimer l’adresse : { $status }

# Contact Edit Card
contact-edit-card-edit-title = Modifier le contact
contact-edit-card-add-title = Ajouter un contact
contact-edit-card-full-name-label = Nom complet
contact-edit-card-preferred-name-label = Nom préféré
contact-edit-card-email-label = Adresse e-mail
contact-edit-card-email-placeholder = E-mail
contact-edit-card-phone-label = Numéro de téléphone
contact-edit-card-phone-placeholder = Téléphone
contact-edit-card-role-title-label = Fonction/Titre
contact-edit-card-save-button = Enregistrer le contact

# Contacts View
contacts-view-add-button = Ajouter un contact
contacts-view-no-role = Aucun rôle spécifié
contacts-view-error-save = Impossible d’enregistrer le contact : { $status }
contacts-view-error-delete = Impossible de supprimer le contact : { $status }

# Delete Address Confirmation Modal
delete-address-confirm-message = Êtes-vous sûr de vouloir supprimer l’adresse : { $address } ?

# Delete Contact Confirmation Modal
delete-contact-confirm-message = Êtes-vous sûr de vouloir supprimer le contact : { $name } { $preferred_name } ?

# Vendor Invoice Drawer
vendor-invoice-drawer-inv-number = Facture n° : { $number }
vendor-invoice-drawer-gross = Brut : { $amount }
vendor-invoice-drawer-outstanding-balance = Solde impayé : { $amount }

# Details View
details-view-error-update = Impossible de mettre à jour la facture : { $status }
details-view-notes-label = Notes :

# Items View
items-view-unknown-gl-account = Compte GL inconnu
items-view-gl-label = GL : { $account }
items-view-net-tax-breakdown = Net : { $net } | Taxe : { $tax }
items-view-add-item-button = + Ajouter un article
items-view-delete-item-title = Supprimer l’article
items-view-delete-item-message = Êtes-vous sûr de vouloir supprimer l’article : { $description } ?
items-view-error-update-items = Impossible de mettre à jour les articles de la facture : { $status }

# Payments View
payments-view-payment-date-label = Date du paiement :
payments-view-bank-account-label = Compte bancaire :
payments-view-reference-label = Référence :
payments-view-make-payment-button = Effectuer le paiement
payments-view-error-parse-payments = Impossible d’analyser les paiements : { $error }
payments-view-error-fetch-payments = Impossible de récupérer les paiements : { $status }
payments-view-error-parse-accounts = Impossible d’analyser les comptes : { $error }
payments-view-error-fetch-accounts = Impossible de récupérer les comptes : { $status }
payments-view-error-make-payment = Impossible d’effectuer le paiement : { $status }

# Item Edit Card
item-edit-card-add-title = Ajouter un article
item-edit-card-edit-title = Modifier l’article
item-edit-card-net-amount-label = Montant net :
item-edit-card-tax-amount-label = Montant des taxes :

# Account Ledger Export
account-ledger-export-report-qualifier = Compte { $account_name } pour la période du { $start_date } au { $end_date }
account-ledger-export-title = Écritures comptables

# Balance Sheet Export
balance-sheet-export-assets-header = Actifs,
balance-sheet-export-total-assets = Total des actifs
balance-sheet-export-liabilities-header = Passifs,
balance-sheet-export-total-liabilities = Total des passifs
balance-sheet-export-equity-header = Capitaux propres,
balance-sheet-export-current-year-earnings = Résultat de l’exercice en cours
balance-sheet-export-total-equity = Total des capitaux propres
balance-sheet-export-total-liabilities-equity = Total passifs et capitaux propres
balance-sheet-export-as-at = Au { $date }

# General Ledger Export
general-ledger-export-period = Période du { $start_date } au { $end_date }

# Profit Loss Export
profit-loss-export-revenue-header = Produits,
profit-loss-export-expenses-header = Charges,

# Trial Balance Export
trial-balance-export-total = Total

# Test keys
test-key = Valeur de test
test-key-override = Valeur de test 2
test-key-args = Bonjour, { $name }!
