/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::components::header::Header;
use crate::components::sidebar::Sidebar;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Children,
}

/// A component that provides the main application layout with a sidebar and header.
/// Any children passed to this component will be rendered in the main content area.
#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    html! {
        <div class="app-shell">
            <Sidebar /> // Your sidebar component
            <main class="main-content">
                <Header /> // Your top bar with the user menu
                    <main class="page-body">
                        { for props.children.iter() }
                    </main>
            </main>
        </div>
    }
}
