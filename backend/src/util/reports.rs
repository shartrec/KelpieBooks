/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use std::io::Cursor;

use rocket::{
    http::{
        ContentType,
        Status,
    },
    response::Responder,
    Request,
    Response,
};
use typst_as_lib::{
    typst_kit_options::TypstKitFontOptions,
    TypstEngine,
};
use typst_assets::fonts;
use typst_library::foundations::{Dict, Value};
use crate::util::{get_static_dir, get_template_dir};

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
    table.header(
        repeat: true,
        {col_headings},
        table.hline(stroke: 0.5pt + gray)
    ),"###,
        col_layout = col_layout,
        col_headings = col_headings
    )
}

pub(crate) fn compile_typst_to_pdf(
    source: String,
    title: &str,
    qualifier: &str,
    org_name: &str,
    template_path: &str,
) -> Result<Vec<u8>, String> {

    // Wrap the report content in a call to load and show the template
    let rep = format!(r###"
    #import "report_template.typ" as t
    // Activate the layout
    #show: t.report_layout.with(
        title: t.report_title,
        org_name: t.org_name,
        report_qualifier: t.report_qualifier
    )
    {}
    "###, source);

    let template = TypstEngine::builder()
        .main_file(rep)
        .with_file_system_resolver(template_path)
        .fonts(fonts())
        .search_fonts_with(TypstKitFontOptions::default())
        .build();

    // Build the inputs
    let mut dict = Dict::new();
    dict.insert("org-name".into(), Value::Str(org_name.into()));
    dict.insert("title".into(), Value::Str(title.into()));
    dict.insert("qualifier".into(), Value::Str(qualifier.into()));

    // Run it
    let doc = template.compile_with_input(dict).output;
    match doc {
        Ok(doc) => {
            let options = Default::default();
            let pdf = typst_pdf::pdf(&doc, &options).expect("Could not generate pdf.");
            Ok(pdf)
        }
        Err(e) => Err(format!("typst::compile() returned an error!: {}", e)),
    }
}
