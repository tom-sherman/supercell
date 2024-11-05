-- Add up migration script here

CREATE TABLE feed_content (
  feed_id TEXT NOT NULL,
  uri TEXT NOT NULL,
  indexed_at INTEGER NOT NULL,
  indexed_at_more INTEGER NOT NULL,
  cid TEXT NOT NULL,
  updated_at DATETIME NOT NULL DEFAULT (datetime('now')),
  PRIMARY KEY (feed_id, uri)
);

CREATE INDEX feed_content_idx_feed ON feed_content(feed_id, indexed_at DESC, indexed_at_more DESC, cid DESC);

CREATE TABLE consumer_control (
  source TEXT NOT NULL,
  time_us VARCHAR NOT NULL,
  updated_at DATETIME NOT NULL DEFAULT (datetime('now')),
  PRIMARY KEY (source)
);

CREATE TABLE verification_method_cache (
  did TEXT NOT NULL,
  multikey TEXT NOT NULL,
  updated_at DATETIME NOT NULL DEFAULT (datetime('now')),
  PRIMARY KEY (did)
);
