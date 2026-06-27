// Import the data supplied as dictionary
#import sys: inputs

#let company-name = inputs.at("company-name")
#let invoice-num = inputs.at("invoice-number")
#let invoice-date = inputs.at("invoice-date")
#let due-date = inputs.at("due-date")

// Define global page and branding properties
#set page(
  paper: "a4",
  margin: (x: 2cm, top: 2.5cm, bottom: 2.5cm),
  header: align(right)[
#grid(  columns: (auto,  1fr),
  align(left)[
    #text(size: 20pt, weight: "bold", fill: rgb("#3b0000"))[#company-name]
  ],
  align(right)[
    #text(size: 10pt, weight: "bold", fill: rgb("#3b0000"))[INVOICE]
  ],
)],
  footer: context [
     #text(size: 8pt, fill: gray)[
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
      [ \ Page - #counter(page).display()]
    )
  ]]
)
#set text(font: "Liberation Sans", size: 10pt, fill: rgb("#2c3e50"))

// Helper functions for currency layout formatting
#let format-currency(amount) = {
  "$" + str(format("{:.2}", amount))
}

// --- Header Section ---
#grid(
  columns: (1fr, 1fr),
  align(left)[

    #v(2pt)
    #text(size: 9pt, fill: gray.darken(30%))[
      123 Accounting Lane \
      Berrima, NSW 2577 \
      Australia
    ]
  ],
  align(right)[
    #text(size: 16pt, weight: "medium", fill: rgb("#2c3e50"))[]
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
#line(length: 100%, stroke: 0.5pt + rgb("#bdc3c7"))
#v(15pt)

// --- Bill To / Ship To Section ---
#grid(
  columns: (1fr, 1fr),
  gutter: 20pt,
  [
    #text(weight: "bold", size: 11pt, fill: rgb("#3b0000"))[Bill To:] \
    #v(3pt)
    *Acme Corporation* \
    Attn: Accounts Payable \
    456 Enterprise Way \
    Sydney, NSW 2000
  ],
  [
    #text(weight: "bold", size: 11pt, fill: rgb("#3b0000"))[Ship To:] \
    #v(3pt)
    *Acme Corp Warehouse* \
    Dock 2, 456 Enterprise Way \
    Sydney, NSW 2000
  ]
)

#v(30pt)

// --- Line Items Table Section ---
#text(weight: "bold", size: 12pt)[Line Items]
#v(5pt)

#table(
  columns: (1fr, auto, auto, auto),
  align: (left, right, right, right),
  stroke: (x, y) => if y == 0 { (bottom: 2pt + rgb("#3b0000")) } else { (bottom: 0.5pt + rgb("#ecf0f1")) },
  fill: (x, y) => if y == 0 { rgb("#b3efe2").lighten(60%) } else if calc.even(y) { rgb("#f8fafc") } else { none },
  inset: 10pt,

  // Header definition
  [*Description*], [*Qty*], [*Unit Price*], [*Amount*],

  // Row lines (Description, Qty, Unit Price, Extension)
  [Software Development Consulting - Phase 3 Setup], [10.00], [\$150.00], [\$1,500.00],
  [Custom API Integration Layer Optimization], [4.50], [\$120.00], [\$540.00],
  [Database Schema Sub-ledger Redesign Package], [1.00], [\$450.00], [\$450.00],
  [Software Development Consulting - Phase 3 Setup], [10.00], [\$150.00], [\$1,500.00],
)

#v(15pt)

// --- Financial Aggregates Breakdown Summary ---
#align(right)[
  #block(width: 40%, breakable: false)[
    #grid(
      columns: (1fr, auto),
      gutter: 10pt,
      align: (left, right),
      [Subtotal:], [\$2,490.00],
      [Tax (GST 10%):], [\$249.00],
      grid.hline(stroke: 1pt + rgb("#bdc3c7")),
      [],[],
      text(weight: "bold")[Total Amount Due:], text(weight: "bold", fill: rgb("#3b0000"))[\$2,739.00]
    )
  ]
]

#v(40pt)

// --- Remittance Advice / Payment Information ---
#block(
  breakable: false,
  fill: rgb("#ecf0f1").lighten(50%),
  inset: 12pt,
  radius: 5%,
  stroke: 1pt + rgb("#bdc3c7").lighten(50%),
  width: 100%
)[
  #text(weight: "bold", fill: rgb("#3b0000"))[How to Pay:] \
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