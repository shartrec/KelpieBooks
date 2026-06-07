/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::{Request, Response};
use std::io::Cursor;
use typst_as_lib::typst_kit_options::TypstKitFontOptions;
use typst_as_lib::TypstEngine;
use typst_assets::fonts;

pub(crate) struct DownloadFile {
    content: Vec<u8>,
    filename: String,
    content_type: ContentType,
}

impl DownloadFile {
    pub(crate) fn new(content: Vec<u8>, filename: String, content_type: ContentType) -> Self {
        DownloadFile {
            content,
            filename,
            content_type,
        }
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for DownloadFile {
    fn respond_to(self, _req: &'r Request<'_>) -> rocket::response::Result<'o> {
        Response::build()
            .header(self.content_type)
            .raw_header(
                "Content-Disposition",
                format!("attachment; filename=\"{}\"", self.filename),
            )
            .sized_body(self.content.len(), Cursor::new(self.content))
            .status(Status::Ok)
            .ok()
    }
}

pub(crate) fn build_table_header(headings: &[String], align_right: &[bool]) -> String {
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

pub(crate) fn wrap_report_layout(
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

pub(crate) fn compile_typst_to_pdf(source: String) -> Result<Vec<u8>, String> {
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

