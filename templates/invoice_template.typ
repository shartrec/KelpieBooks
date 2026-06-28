// Import the data supplied as dictionary
#import sys: inputs

// Some style elements
#set text(font: "Liberation Sans", size: 10pt, fill: rgb("#2c3e50"))
#let brand_color = rgb("#6b0000")
#let line_color = rgb("#bdc3c7")
#let shadow_color = rgb("#e5f5f5")

// Extract basic invoice data into variables, for simplicity
#let company-name = inputs.at("company-name")
#let invoice-num = inputs.at("invoice-number")
#let invoice-date = inputs.at("invoice-date")
#let due-date = inputs.at("due-date")
#let invoice-net = inputs.at("invoice-net")
#let invoice-tax = inputs.at("invoice-tax")
#let invoice-gross = inputs.at("invoice-gross")

// Define global page and branding properties
#set page(
  paper: "a4",
  margin: (x: 1.2cm, top: 2.0cm, bottom: 2.5cm),
  header: align(right)[
#grid(  columns: (auto,  1fr),
  align(left)[
    #text(size: 20pt, weight: "bold", fill: brand_color)[#company-name]
  ],
  align(right)[
    #text(size: 14pt, weight: "bold", fill: brand_color)[Tax Invoice]
  ],
)],
  footer: context [
     #text(size: 8pt, fill: gray.darken(50%))[
     #grid(
      columns: (1fr, auto, 1fr),
      gutter: 8pt,
      align: (right, right),
      [],
      [#align(center)[
         [
            Thank you for your business! | Payment terms: Net 30 days \
            Kelpie Books Ltd · 123 Accounting Lane, Suite 400 · support\@kelpiebooks.com
          ]
        ]
      ],
      [ \ Page:  #counter(page).display() of #counter(page).final().at(0)]
    )
  ]]
)

// --- Header Section ---
#grid(
  columns: (1fr, 1fr),
  align(left)[

    #v(2pt)
    #text(size: 9pt, fill: brand_color.lighten(30%))[
      123 Accounting Lane \
      Berrima, NSW 2577 \
      Australia
    ]
  ],
  align(right)[
    #text(size: 16pt, weight: "medium")[]
    #v(5pt)
    #grid(
      columns: (auto, auto),
      gutter: 8pt,
      align: (right, right),
      [*Invoice No:*], [#invoice-num],
      [*Date:*], [#invoice-date],
      [*Due Date:*], [#due-date],
    )
  ]
)

#v(30pt)
#line(length: 100%, stroke: 0.5pt + line_color)
#v(15pt)

// --- Bill To / Ship To Section ---
#let bill_to = inputs.at("bill_to")
#let ship_to = inputs.at("ship_to")


#grid(
  columns: (1fr, 30%, 1fr),
  gutter: 20pt,
  [
    #text(weight: "bold", size: 11pt, fill: brand_color)[Bill To:] \
    #v(3pt)
    #let attn = bill_to.at("attn")
    #if attn != "" [
      Attn: #attn \
    ]
    *#bill_to.at("name")* \
    #bill_to.at("addr_line1") \
    #let l2 = bill_to.at("addr_line2")
    #if l2 != "" [
      #l2
    ]
    #bill_to.at("city"), #bill_to.at("state") #bill_to.at("post_code")
  ],
  [],
  [
    #text(weight: "bold", size: 11pt, fill: brand_color)[Ship To:] \
    #v(3pt)
    #let attn = ship_to.at("attn")
    #if attn != "" [
      Attn: #attn \
    ]
    *#ship_to.at("name")* \
    #ship_to.at("addr_line1") \
    #let l2 = bill_to.at("addr_line2")
    #if l2 != "" [
      #l2
    ]
    #ship_to.at("city"), #ship_to.at("state") #ship_to.at("post_code")
  ]
)

#v(20pt)

#let lines = inputs.at("lines")


#table(
  columns: (1fr, auto, auto, auto, auto, auto),
  align: (left, right, right, right, right, right),
  stroke: (x, y) => if y == 0 { (bottom: 2pt + brand_color) } else { none },
  fill: (x, y) => if y == 0 { shadow_color } else if calc.even(y) { shadow_color.lighten(60%) } else { none },
  inset: 10pt,

  // Header definition
  [*Description*], [*Qty*], [*Unit*], [*Net*], [*Tax*], [*Amount*],

  ..lines.map(line => (
    // Row lines (Description, Qty, Unit Price, Extension)
    [#line.at("code") - #line.at("name")], [#line.at("qty")], [\$#line.at("unit_price")], [\$#line.at("net")], [\$#line.at("tax")], [\$#line.at("gross")],
  )).flatten()
)

#v(15pt)

// --- Financial Aggregates Breakdown Summary ---
#align(right)[
  #block(width: 40%, breakable: false)[
    #grid(
      columns: (1fr, auto),
      gutter: 10pt,
      align: (left, right),
      [Subtotal:], [\$#invoice-net],
      [Tax (GST 10%):], [\$#invoice-tax],
      grid.hline(stroke: 1pt + line_color),
      [],[],
      text(weight: "bold")[Total Amount Due:], text(weight: "bold", fill: brand_color)[\$#invoice-gross]
    )
  ]
]

#v(40pt)

// --- Remittance Advice / Payment Information ---
#block(
  breakable: false,
  fill: shadow_color.lighten(60%),
  inset: 12pt,
  radius: 5%,
  stroke: 1pt + line_color,
  width: 100%,
  above: 1fr
)[
  #text(weight: "bold", fill: brand_color)[How to Pay:] \
  #v(2pt)
  Please remit bank transfer payments directly to the account details below, citing your invoice number as the reference descriptor:
  #v(4pt)
  #grid(
    columns: (auto, auto),
    gutter: 6pt,
    [*Bank:*], [Global Enterprise Bank Australia],
    [*BSB:*], [123-456],
    [*Account No:*], [9876 5432 10],
    [*Reference:*], [#invoice-num]
  )
]