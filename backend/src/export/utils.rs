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
use typst_as_lib::typst_kit_options::TypstKitFontOptions;
use typst_as_lib::TypstEngine;
use typst_assets::fonts;

pub fn build_table_header(headings: &[String], align_right: &[bool]) -> String {
    // Generate column layout: first is auto, rest are 1fr
    // For 3 headings, this produces: "auto, 1fr, 1fr"
    let col_layout = std::iter::once("auto")
        .chain(std::iter::repeat("1fr").take(headings.len() - 1))
        .collect::<Vec<_>>()
        .join(", ");

    // Format headings as Typst bold blocks: [*Heading*]
    let col_headings = headings
        .iter()
        .enumerate()
        .map(|(i, s)| {
            format!(
                "{}[*{}*]",
                if align_right[i] { "align(right)" } else { "" },
                s
            )
        })
        .collect::<Vec<_>>()
        .join(", ");

    // Use a raw string with named placeholders for readability
    format!(
        r###"#table(
    columns: ({col_layout}),
    fill: (x, y) => {{
        if y == 0 {{ rgb("#f4f7f6") }}
        else if calc.even(y) {{ rgb("#f4fbff") }}
        else {{ white }}
    }},
    table.header(
        repeat: true,
        {col_headings},
        table.hline(stroke: 0.5pt + gray)
    ),"###,
        col_layout = col_layout,
        col_headings = col_headings
    )
}

fn get_template() -> String {
    let template = r###"

#let tab_h_color = "#f4f7f6"
#let tab_odd_color = "#f4fbff"

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
        footer: context { [
            #set text(8pt, fill: gray)
            #line(length: 100%, stroke: 0.5pt + gray)
            #grid(
                columns: (1fr, 1fr),
                align(left)[#title],
                align(right)[Page #counter(page).display()]
            )
        ]}
    )
    set text(font: ("New Computer Modern", "DejaVu Sans", "sans-serif"), size: 10pt)

    set table(
      fill: (x, y) => {
          if y == 0 { rgb(tab_h_color) }
          else if calc.even(y) { rgb(tab_odd_color) }
          else { white }
      },
      stroke: (x, y) => {
        none
      }
    )

    // Header section inside the layout
    grid(
        columns: (1fr, auto),
        text(size: 18pt, weight: "bold")[#title - #org_name],
        align(right + bottom)[#text(size: 10pt, style: "italic")[#report_qualifier]]
    )

    body
}
"###;

    template.to_string()
}

pub fn wrap_report_layout(
    org_name: Option<&str>,
    report_title: &str,
    report_qualifier: &str,
    body: &str,
) -> String {
    format!(
        r#"
        {template}
        // Definitions at the top
        #let org_name = "{org_name}"
        #let report_title = "{report_title}"
        #let report_qualifier = "{report_qualifier}"

        // Activate the layout
        #show: report_layout.with(
            title: report_title,
            org_name: org_name,
            report_qualifier: report_qualifier,
        )
        {body}
        "#,
        template = get_template(),
        org_name = org_name.unwrap_or(""),
        report_title = report_title,
        report_qualifier = report_qualifier,
        body = body
    )
}

pub fn compile_typst_to_pdf(source: String) -> Result<Vec<u8>, String> {
    let template = TypstEngine::builder()
        .main_file(source)
        .fonts(fonts())
        .search_fonts_with(TypstKitFontOptions::default())
        .build();

    // Run it
    let doc = template.compile().output;
    match doc {
        Ok(doc) => {
            let options = Default::default();
            let pdf = typst_pdf::pdf(&doc, &options).expect("Could not generate pdf.");
            Ok(pdf)
        }
        Err(e) => Err(format!("typst::compile() returned an error!: {}", e)),
    }
}
