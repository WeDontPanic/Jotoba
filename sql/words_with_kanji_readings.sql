SELECT dict.id, dict.sequence as sequence, dict.reading, dict.kanji, dict.no_kanji, dict.priorities, dict.information, dict.kanji_info, dict.jlpt_lvl, dict.is_main, dict.accents, dict.furigana
FROM dict JOIN
  Dict AS D2 ON D2.sequence = dict.sequence
WHERE 
  (d2.reading &@ $1 OR d2.reading &@ $2) AND dict.reading &@ $3 AND dict.kanji = true AND (dict.is_main = $4 OR d2.is_main = $4)
ORDER BY
  LENGTH(d2.reading), d2.priorities
