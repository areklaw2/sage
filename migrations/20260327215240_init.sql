CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE embeddings (
  id           BIGSERIAL PRIMARY KEY,
  source       TEXT NOT NULL,
  file         TEXT,
  section      TEXT,
  fn_name      TEXT,
  line_start   INT,
  line_end     INT,
  content      TEXT NOT NULL,
  content_hash TEXT NOT NULL,
  embedding    vector(768),
  created_at   TIMESTAMPTZ DEFAULT now()
);

CREATE UNIQUE INDEX ON embeddings (source, content_hash);
CREATE INDEX ON embeddings USING hnsw (embedding vector_cosine_ops);
