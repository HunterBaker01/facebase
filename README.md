# Photo Organizer

> **Learning project** — built to explore Rust, PostgreSQL vector search, and face recognition. Not meant to outperform on-the-market products.

CLI tool that registers faces and recognizes people in photos using facial embeddings stored in PostgreSQL with pgvector. The Rust binary handles the CLI and database logic; a Python script generates embeddings via [DeepFace](https://github.com/serengil/deepface).

## How it works

1. Register a person by providing a photo and a name — the face embedding is stored in Postgres
2. Recognize a face in a new photo — the embedding is compared against stored ones using cosine distance

## Prerequisites

- Rust (via [rustup](https://rustup.rs))
- PostgreSQL with the [pgvector](https://github.com/pgvector/pgvector) extension
- Python 3 with DeepFace: `pip install deepface`

## Database setup

```sql
CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE people (
    id   SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    notes TEXT
);

CREATE TABLE face_embeddings (
    id               SERIAL PRIMARY KEY,
    person_id        INTEGER REFERENCES people(id),
    embedding        vector(512),
    source_image_path TEXT
);
```

## Usage

```bash
# Register a person
cargo run -- register path/to/photo.jpg --name "Alice"

# Optionally add notes
cargo run -- register path/to/photo.jpg --name "Alice" --notes "sister"

# Recognize a face
cargo run -- recognize path/to/unknown.jpg
```

The tool connects to a local PostgreSQL database named `photos` using the current `$USER`.
