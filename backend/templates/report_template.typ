#import sys: inputs

#let org_name = inputs.at("org-name")
#let report_title = inputs.at("title")
#let report_qualifier = inputs.at("qualifier")

#let tab_h_color = rgb("#f4f7f6")
#let tab_odd_color = rgb("#f4fbff")

#let report_layout(
    title: "",
    org_name: "",
    report_qualifier: "",
    body
) = {
    set page(
        paper: "a4",
        margin: (top: 2.5cm, bottom: 2cm, x: 1.5cm),
        flipped: false,   // Set to true for landscape
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
                align(right)[Page #counter(page).display() of #counter(page).final().at(0)]
            )
        ]}
    )
    set text(font: "Noto Sans", size: 10pt)

    set table(
      fill: (x, y) => {
          if y == 0 { tab_h_color }
          else if calc.even(y) { tab_odd_color }
          else { white }
      },
      stroke: (x, y) => {
        none
      }
    )

    // Header section inside the layout
    grid(
        columns: (1fr, auto),
        text(size: 16pt, weight: 700)[#title - #org_name],
        align(right + bottom)[#text(size: 10pt, style: "italic")[#report_qualifier]]
    )

    body
}
