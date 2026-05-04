// Минималистичный дизайн одной ячейки бланка.
//
// Важно: сигнатура должна быть совместима с blanks.typ,
// чтобы дизайн можно было переключать из UI без изменений кода.

#let blank_cell(team_label, event, question_num, pic, font_name) = [
  #box(width: 95%, height: 95%, [
    #place(left + top, text(size: 9pt, font: font_name, fill: rgb("#000000"), team_label))

    #if pic != none [
      #let logo_box_width = 18%
      #let logo_box_height = 18%
      #let logo_dx = -2%
      #let logo_dy = -4%
      #place(
        right + top,
        dx: logo_dx,
        dy: logo_dy,
        box(
          width: logo_box_width,
          height: logo_box_height,
          clip: true,
          align(center + horizon, pic),
        ),
      )
    ]

    #place(right+bottom, text(
      size: 16pt,
      font: font_name,
      weight: "bold",
      fill: rgb("#000000"),
      [#question_num],
    ))

    #place(right + bottom, text(size: 10pt, font: font_name, fill: rgb("#000000"), event))
  ])
]
