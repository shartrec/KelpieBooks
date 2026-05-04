/*
 * Copyright (c) 2026. Trevor Campbell and others.
 *
 * This file is part of KelpieBooks.
 *
 * KelpieBooks is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License,or
 * (at your option) any later version.
 *
 * KelpieBooks is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with KelpieBooks; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 *
 * Contributors:
 *      Trevor Campbell
 *
 */

pub fn wrap_report_layout(org_name: &str, report_title: &str,  report_qualifier: &str,  body: &str) -> String {

    let topmatter = r###"

#let report_layout(title: "", org_name: "", report_qualifier: "", body: []) = {
    set page(
        paper: "a4",
        margin: (top: 2.5cm, bottom: 2cm, x: 1.5cm),
        header: [ ... ], // your header logic
        footer: [ ... ]  // your footer logic
    )
    body // This is where the table will be injected
}

#let report_layout(
    title: "",
    org_name: "",
    report_qualifier: "",
    body
) = {
    set page(
        paper: "a4",
        margin: (top: 2.5cm, bottom: 2cm, x: 1.5cm),
        header: [
            #set text(8pt, fill: gray)
            #grid(
                columns: (1fr, 1fr),
                align(left)[Kelpie Books],
                align(right)[#datetime.today().display()]
            )
            #line(length: 100%, stroke: 0.5pt + gray)
        ],
        footer: [
            #set text(8pt, fill: gray)
            #line(length: 100%, stroke: 0.5pt + gray)
            #grid(
                columns: (1fr, 1fr),
                align(left)[#title],
                align(right)[Page 3]
            )
        ]
    )
    
    set text(font: ("Linux Libertine", "DejaVu Sans", "sans-serif"), size: 10pt)
    
    // Header section inside the layout
    grid(
        columns: (1fr, 1fr),
        text(size: 18pt, weight: "bold")[#title - #org_name],
        align(right + bottom)[#text(size: 10pt, style: "italic")[#report_qualifier]]
    )

    body // This is where your table is injected
}

// Activate the layout
#show: report_layout.with(
    title: report_title,
    org_name: org_name,
    report_qualifier: report_qualifier,
)

    "###;

    format!(
        r#"
        // Definitions at the top
        #let org_name = "{org_name}"
        #let report_title = "{report_title}"
        #let report_qualifier = "{report_qualifier}"

        {topmatter}

        {body}
        "#,
        org_name = org_name,
        report_title = report_title,
        report_qualifier = report_qualifier,
        topmatter = topmatter,
        body = body
    )
}