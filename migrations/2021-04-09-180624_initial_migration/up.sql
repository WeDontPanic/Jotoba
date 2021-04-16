CREATE TABLE dict (
  id SERIAL PRIMARY KEY,
  sequence INTEGER NOT NULL,
  reading TEXT NOT NULL,
  kanji boolean NOT NULL,
  no_kanji boolean NOT NULL,
  priorities TEXT[],
  information TEXT[]
);
CREATE INDEX index_reading_dict ON dict (reading);
CREATE INDEX index_seq_dict ON dict (sequence);

CREATE TABLE sense (
  id SERIAL PRIMARY KEY,
  sequence INTEGER NOT NULL,
  language TEXT NOT NULL,
  gloss_pos INTEGER NOT NULL,
  gloss TEXT NOT NULL,
  misc TEXT,
  part_of_speech TEXT[],
  dialect TEXT,
  xref TEXT,
  gtype TEXT,
  field TEXT,
  information TEXT,
  antonym TEXT
);
CREATE INDEX index_seq_sense ON sense (sequence);
CREATE INDEX index_gloss_sense ON sense (gloss);
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
  natori TEXT[]
);
CREATE INDEX index_literal_kanji ON kanji (literal);
