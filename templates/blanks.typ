
// ─── Configuration ────────────────────────────────────────────────────────────
// Edit these values, or generate via the GUI (app.py).

#import sys: inputs

#let start_team  = inputs.start_team
#let finish_team = inputs.finish_team
#let cols        = inputs.cols
#let rows        = inputs.rows
#let landscape   = inputs.landscape
#let location    = inputs.location
#let team_prefix = inputs.team_prefix
#let font_name   = inputs.font_name
#let logo_path   = inputs.logo_path
#let team_names  = inputs.team_names
#let logo        = if logo_path == "" { none } else { image(logo_path) }

// ─── Optional font ─────────────────────────────────────────────────────────────
#if font_name != "" {
  set text(font: font_name)
}

// ─── Layout helpers ────────────────────────────────────────────────────────────

#let blank_cell(team_label, event, question_num, pic) = [
  #box(width: 90%, height: 80%, [
    #place(left  + top,    text(size: 10pt, team_label))
    #if pic != none [
      #place(right + top,  move(dx: 10%, dy: -15%, box(height: 70%, pic)))
    ]
    #place(center + horizon, text(
      size: 50pt,
      fill: color.black.transparentize(80%),
      [#question_num],
    ))
    #place(right + bottom, event)
  ])
]

#let blank_page(team_label, event, pic, c, r, q0) = {
  set page(margin: 0cm, flipped: landscape)
  let cells = range(q0, c * r + q0).map(n =>
    align(center + horizon, blank_cell(team_label, event, n, pic))
  )
  grid(
    columns: (100% / c,) * c,
    rows:    (100% / r,) * r,
    gutter:  0cm,
    stroke:  1pt + black,
    ..cells,
  )
}

// ─── Output ────────────────────────────────────────────────────────────────────

#if team_names.len() > 0 {
  for i in range(0, team_names.len()) {
    let team_number = i + 1
    let team_name = team_names.at(i)
    let label = team_prefix + str(team_number) + " " + team_name
    blank_page(label, location, logo, cols, rows, 1)
  }
} else {
  for i in range(start_team, finish_team + 1) {
    let label = team_prefix + str(i)
    blank_page(label, location, logo, cols, rows, 1)
  }
}


