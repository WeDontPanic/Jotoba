SELECT kanji_element.kanji_id, search_radical.literal FROM kanji_element 
  JOIN search_radical ON search_radical.id = kanji_element.search_radical_id 
    WHERE kanji_element.kanji_id in 
      (SELECT kanji_element.kanji_id FROM search_radical JOIN kanji_element 
        ON kanji_element.search_radical_id = search_radical.id where search_radical.literal = $1)
