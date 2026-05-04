// Служебный runtime-файл.
//
// Во время рендера приложение подменяет этот файл выбранным дизайном,
// поэтому сигнатура blank_cell должна совпадать с остальными дизайнами.

#let blank_cell(team_label, event, question_num, pic, font_name) = [
  #box(width: 90%, height: 80%, [
    #place(left + top, text(size: 10pt, font: font_name, team_label))
    #if pic != none [
      #let logo_box_width = 22%
      #let logo_box_height = 22%
      #let logo_dx = -2%
      #let logo_dy = 4%
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
    #place(center + horizon, text(
      size: 50pt,
      font: font_name,
      fill: color.black.transparentize(80%),
      [#question_num],
    ))
    #place(right + bottom, text(font: font_name, event))
  ])
]
