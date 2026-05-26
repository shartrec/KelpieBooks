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

