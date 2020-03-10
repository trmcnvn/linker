CREATE TABLE links (
  id serial primary key,
  external_url text NOT NULL,
  short_id text NOT NULL,
  created_at timestamp(6) without time zone DEFAULT current_timestamp NOT NULL
);
CREATE UNIQUE INDEX index_short_id ON links USING btree (short_id);