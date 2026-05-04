
// Основной шаблон генерации бланков.
//
// Ответственность файла:
// 1) читать входы из sys.inputs,
// 2) строить сетку и нумерацию,
// 3) подключать активный дизайн ячейки из designs/current.typ.

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
#let sheets_per_team = inputs.sheets_per_team
#let rows_first_sheet = inputs.rows_first_sheet
#let rows_second_sheet = inputs.rows_second_sheet
#let logo        = if logo_path == "" {
  none
} else {
  image(logo_path, width: 100%, height: 100%, fit: "contain")
}

#import "designs/current.typ": blank_cell

// ─── Optional font ─────────────────────────────────────────────────────────────
// Глобально задаём шрифт документа, если он передан из приложения.
#if font_name != "" {
  set text(font: font_name)
}

// ─── Layout helpers ────────────────────────────────────────────────────────────

// Рисует одну страницу с сеткой c×r.
// q0 — стартовый номер вопроса на этой странице.
#let blank_page(team_label, event, pic, c, r, q0) = {
  set page(margin: 0cm, flipped: landscape)
  let cells = range(q0, c * r + q0).map(n =>
    align(center + horizon, blank_cell(team_label, event, n, pic, font_name))
  )
  grid(
    columns: (100% / c,) * c,
    rows:    (100% / r,) * r,
    gutter:  0cm,
    stroke:  1pt + black,
    ..cells,
  )
}

// Рисует бланк команды на 1 или 2 листах.
// Во втором листе нумерация продолжается с first_count + 1.
#let blank_set(team_label, event, pic, c) = {
  let first_count = c * rows_first_sheet
  blank_page(team_label, event, pic, c, rows_first_sheet, 1)

  if sheets_per_team == 2 and rows_second_sheet > 0 {
    blank_page(team_label, event, pic, c, rows_second_sheet, first_count + 1)
  }
}

// ─── Output ────────────────────────────────────────────────────────────────────
// Если есть список команд из файла — используем его,
// иначе генерируем по числовому диапазону start_team..finish_team.

#if team_names.len() > 0 {
  for i in range(0, team_names.len()) {
    let team_number = i + 1
    let team_name = team_names.at(i)
    let label = team_prefix + str(team_number) + " " + team_name
    blank_set(label, location, logo, cols)
  }
} else {
  for i in range(start_team, finish_team + 1) {
    let label = team_prefix + str(i)
    blank_set(label, location, logo, cols)
  }
}


