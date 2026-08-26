// Import the data supplied as dictionary
#import sys: inputs

// Some style elements
#set text(font: "Noto Sans", size: 10pt, fill: rgb("#2c3e50"))
#let brand_color = rgb("#6b0000")
#let line_color = rgb("#bdc3c7")
#let shadow_color = rgb("#e5f5f5")

// Extract basic order data into variables, for simplicity
#let company-name = inputs.at("company-name")
#let order-num = inputs.at("order-number")
#let order-date = inputs.at("order-date")
#let due-date = inputs.at("due-date")
#let order-net = inputs.at("order-net")
#let order-tax = inputs.at("order-tax")
#let order-gross = inputs.at("order-gross")

// Define global page and branding properties
#set page(
  paper: "a4",
  margin: (x: 1.2cm, top: 2.0cm, bottom: 2.5cm),
  flipped: true,
  header: align(right)[
#grid(  columns: (auto,  1fr),
  align(left)[
    #text(size: 20pt, weight: "bold", fill: brand_color)[#company-name]
  ],
  align(right)[
    #text(size: 14pt, weight: "bold", fill: brand_color)[Picking Slip]
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
            Thank you for your business! \
            Kelpie Books Ltd · 123 Accounting Lane, Suite 400 · support\@kelpiebooks.com
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
      [*Order No:*], [#order-num],
      [*Date:*], [#order-date],
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
  columns: (1fr, auto, auto, auto, auto),
  align: (left, right, right, right, right, right),
  stroke: (x, y) => if y == 0 { (bottom: 2pt + brand_color) } else { 0.5pt },
  fill: (x, y) => if y == 0 { shadow_color } else if calc.even(y) { shadow_color.lighten(60%) } else { none },
  inset: 10pt,

  // Header definition
  [*Description*], [*Qty*], [*UoM*], [*Location*], [picked],

  ..lines.map(line => (
    // Row lines (Description, Qty, Unit Price, Extension)
    [#line.at("code") - #line.at("name")], [#line.at("qty")], [#line.at("uom", default: "")], [#line.at("location", default: "")], [ ],
  )).flatten()
)

#v(15pt)

