## 1. The "Smart" Landing Page

Instead of a hard redirect to a login screen, your root URL (/) should serve a lightweight "Welcome" or "Gateway" page.

    Primary Action: "Sign In" (For existing users).

    Secondary Action: "Create New Organization" (For new users).

How it works technically:
The frontend checks for a local JWT or session cookie.

    If found: Redirect to /dashboard.

    If not found: Show the Gateway.

## 2. The "Invite-Only" vs. "Self-Serve" Logic

In accounting software, you usually have two scenarios for a new user:

    The Trailblazer: They are the first person there, creating a brand new Organization.

    The Employee: They were invited to an existing Organization.

The Neat Solution:
Use a "Join or Create" step immediately after the user creates their account.

    Step 1: Identity. User enters email/password. Now they exist in the users table, but their organization_id is NULL.

    Step 2: The Fork in the Road.

        Option A: "Create a New Organization" → (Triggers your COA template flow).

        Option B: "I have an Invite Code" → (Links them to an existing Org).

## 3. Handling the "First Ever" Run

If you want the system to be "zero-config," you can add a small check in your Backend or Frontend.

The "Boot" Check:
When the app loads, the frontend pings a "Health" or "Status" endpoint (e.g., /api/v1/status).

    If the backend returns initialized: false (meaning the organizations table is empty), the UI automatically hides the Login button and shows a prominent "Setup System" button.

## 4. Revised URL Structure

A clean way to organize this in Rocket/Yew is by using clear paths:

    /login — Standard login for anyone.

    /register — Generic user creation.

    /onboard — The specific multi-step wizard for setting up a new Org (Name, COA Template).

    /dashboard — Only accessible once user.organization_id is NOT NULL.
