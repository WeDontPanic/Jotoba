CREATE TABLE dict (
  id SERIAL PRIMARY KEY,
  sequence INTEGER NOT NULL,
  reading TEXT NOT NULL,
  kanji boolean NOT NULL,
  no_kanji boolean NOT NULL,
  priorities TEXT[],
  information INTEGER[],
  kanji_info INTEGER[],
  jlpt_lvl INTEGER
);
CREATE INDEX index_reading_dict ON dict (reading);
CREATE INDEX index_seq_dict ON dict (sequence);

CREATE TABLE sense (
  id SERIAL PRIMARY KEY,
  sequence INTEGER NOT NULL,
  language INTEGER NOT NULL,
  gloss_pos INTEGER NOT NULL,
  gloss TEXT NOT NULL,
  misc TEXT,
  part_of_speech TEXT[],
  dialect TEXT,
  xref TEXT,
  gtype INTEGER,
  field TEXT,
  information TEXT,
  antonym TEXT
);
CREATE INDEX index_seq_sense ON sense (sequence);
CREATE INDEX index_gloss_sense ON sense (gloss text_pattern_ops);
CREATE INDEX index_lang_sense ON sense (language);

CREATE TABLE kanji (
  id SERIAL PRIMARY KEY,
  literal CHAR(1) NOT NULL,
  meaning TEXT[] NOT NULL,
  grade INTEGER,
  stroke_count INTEGER NOT NULL,
  frequency INTEGER,
  jlpt INTEGER,
  variant TEXT[],
  onyomi TEXT[],
  kunyomi TEXT[],
  chinese TEXT,
  korean_r TEXT[],
  korean_h TEXT[],
  natori TEXT[],
  kun_dicts INTEGER[]
);
CREATE INDEX index_literal_kanji ON kanji (literal);

CREATE TABLE name (
  id SERIAL PRIMARY KEY,
  sequence INTEGER NOT NULL,
  kana TEXT NOT NULL,
  kanji TEXT,
  transcription TEXT NOT NULL,
  name_type INTEGER[],
  xref TEXT
);
CREATE INDEX index_kana_name ON name (kana);
CREATE INDEX index_kanji_name ON name (kanji);
CREATE INDEX index_transcription_name ON name (transcription);

CREATE OR REPLACE FUNCTION is_kanji(IN inp text)
 RETURNS boolean AS
 $BODY$
     SELECT
         inp ~ '^[\x3400-\x4DB5\x4E00-\x9FCB\xF900-\xFA6A]*$'
 $BODY$
 LANGUAGE sql
 IMMUTABLE
 STRICT;

CREATE OR REPLACE FUNCTION is_kana(IN inp text)
 RETURNS boolean AS
 $BODY$
     SELECT
         inp ~ '^[ぁ-んァ-ン]*$'
 $BODY$
 LANGUAGE sql
 IMMUTABLE
 STRICT;

CREATE OR REPLACE FUNCTION is_hiragana(IN inp text)
 RETURNS boolean AS
 $BODY$
     SELECT
         inp ~ '^[ぁ-ゔゞ゛゜ー]*$'
 $BODY$
 LANGUAGE sql
 IMMUTABLE
 STRICT;

CREATE OR REPLACE FUNCTION is_katakana(IN inp text)
 RETURNS boolean AS
 $BODY$
     SELECT
         inp ~ '^[ァ-・ヽヾ゛゜ー]*$'
 $BODY$
 LANGUAGE sql
 IMMUTABLE
 STRICT;

CREATE OR REPLACE FUNCTION ends_with_hiragana(IN inp text)
 RETURNS boolean AS
 $BODY$
     SELECT
         inp ~ '[ぁ-ゔゞ゛゜ー]+$'
 $BODY$
 LANGUAGE sql
 IMMUTABLE
 STRICT;

CREATE OR REPLACE FUNCTION get_kun_dicts(i integer)
RETURNS table (id INTEGER, sequence INTEGER, reading TEXT, kanji boolean, no_kanji boolean, priorities TEXT[], information INTEGER[], kanji_info INTEGER[], jlpt_lvl INTEGER)  AS $$
  select * from dict where reading in ( SELECT literal || SUBSTRING(UNNEST(kunyomi) from POSITION('.' in  UNNEST(kunyomi))+1) from kanji where kanji.id = i)
$$
LANGUAGE sql stable;

CREATE OR REPLACE FUNCTION find_kanji_by_meaning(mea TEXT)
  RETURNS set  "kanji" AS $$
    select id, literal, meaning, grade, stroke_count, frequency, jlpt, variant, onyomi, kunyomi, chinese, korean_r, korean_h, natori, kun_dicts
  from (select *, unnest(meaning) m from kanji) x order by mea <-> m limit 4
  $$
 LANGUAGE sql stable;
