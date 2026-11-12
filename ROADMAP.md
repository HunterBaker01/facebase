# Facebase Roadmap

_Last updated: 2026-11-12_

## Vision

Recognize people in front of a camera and show each of them a personalized display (weather, to-dos, calendar, email, etc.). Unknown faces get a generic guest view.

## Status at a Glance

| # | Milestone | Status | Owner |
|---|-----------|--------|-------|
| M1 | Register | 🟡 In progress | Hunter |
| M2 | Recognition | 🟡 In progress | Hunter |
| M3 | Personalization (LLM) | ⚪ Not started | Hunter |
| M4 | Display | ⚪ Not started | Hunter |

Legend: ⚪ Not started · 🟡 In progress · 🟢 Done · 🔴 Blocked

---

## M1 — Register

**Goal:** Store a person's face embeddings in the database, linked to their profile.

**Current state:** `register` CLI command works. It embeds a photo (ArcFace via DeepFace), upserts the person into `people`, and inserts the embedding into `face_embeddings`.

**Tasks**
- [x] CLI `register` command with `--name` and `--notes`
- [x] Store embeddings in Postgres/pgvector
- [ ] Make register photos temporary and delete them after registration
- [ ] Handle photos with zero or multiple faces (currently uses `embedding[0]` only)

**Definition of done:** A person can be registered from one or more photos, and no source photos remain on disk afterward.

---

## M2 — Recognition

**Goal:** Compare an input face to the stored embeddings. If the distance is under the threshold, return the closest match; otherwise return guest.

**Current state:** `recognize` CLI command works, using cosine distance (`<=>`) with a hardcoded threshold of `0.4`.

**Tasks**
- [x] Nearest-neighbor lookup with pgvector
- [x] Fall back to "Unknown" above the threshold
- [ ] Return person ID (not just name) for use in M3
- [ ] Make the threshold configurable
- [ ] Tune the threshold on real photos

**Definition of done:** Given a photo, the system reliably returns the correct person ID or `guest`.

---

## M3 — Personalization (LLM)

**Goal:** Feed the recognized ID to an LLM, which matches it to a profile and generates a personalized response.

**Tasks**
- [ ] Define the profile data model (what each person's profile contains)
- [ ] Pass the recognized ID to the LLM
- [ ] LLM matches the ID to a profile and produces a personalized response
- [ ] Integrate data sources (weather, to-do, calendar, email, etc)

**Definition of done:** A recognized ID produces a personalized response; `guest` produces a generic one.

---

## M4 — Display

**Goal:** Show the personalized info from the LLM on a display.

**Tasks**
- [ ] Choose display target (web page, desktop app, hardware screen)
- [ ] Render the LLM response
- [ ] Guest view

**Definition of done:** Stepping in front of the camera shows the right person's display.

---

## Open Questions

| # | Question | Milestone | Decision |
|---|----------|-----------|----------|

## Out of Scope

- Competing with commercial face-recognition products (this is a learning project).

## Changelog

- **2026-11-12:** Converted `plan.md` into this roadmap.
