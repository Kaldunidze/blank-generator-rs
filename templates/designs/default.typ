// Базовый (дефолтный) дизайн одной ячейки бланка.
//
// Контракт функции обязан совпадать во всех дизайнах:
// blank_cell(team_label, event, question_num, pic, font_name)

#let blank_cell(team_label, event, question_num, pic, font_name) = [

  #align(center + horizon, box(width: 90%, height: 90%, [
    #place(left + top, text(size: 10pt, font: font_name, team_label))
    #if pic != none {
      let logo_box_width = 62%
      let logo_box_height = 62%
      let logo_dx = 15%
      let logo_dy = -10%
      place(
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
    }
    #place(center + horizon, text(
      size: 50pt,
      font: font_name,
      fill: color.black.transparentize(80%),
      [#question_num],
    ))
    #place(right + bottom, text(font: font_name, event))
  ]))
]
