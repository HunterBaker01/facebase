# Facebase Roadmap

_Last updated: 2026-11-18_

## Vision

Recognize people in front of a camera and show each of them a personalized display (weather, to-dos, calendar, email, etc.). Unknown faces get a generic guest view.

## Status at a Glance

| # | Milestone | Status | Owner |
|---|-----------|--------|-------|
| M1 | Register | 🟢 Done | Hunter |
| M2 | Recognition | 🟢 Done | Hunter |
| M3 | Display | 🟢 Done | Hunter |
| M4 | Live Rendering |  🟢 Done | Hunter | 
| M5 | Working Pipeline | 🟡 In progress | Hunter | 
| M6 | Clean Up | ⚪ Not Started | Hunter | 

Legend: ⚪ Not started · 🟡 In progress · 🟢 Done · 🔴 Blocked

---

## M1 — Register

**Goal:** Store a person's face embeddings in the database, linked to their profile.

**Current state:** `register` CLI command works. It embeds a photo (ArcFace via DeepFace), upserts the person into `people`, and inserts the embedding into `face_embeddings`.

**Tasks**
- [x] CLI `register` command with `--name` and `--notes`
- [x] Store embeddings in Postgres/pgvector
- [x] Make register photos temporary and delete them after registration
- [x] Handle photos with zero or multiple faces (currently uses `embedding[0]` only)

**Definition of done:** A person can be registered from one or more photos, and no source photos remain on disk afterward.

---

## M2 — Recognition

**Goal:** Compare an input face to the stored embeddings. If the distance is under the threshold, return the closest match; otherwise return guest. Feed the returned profile into an llm.

**Current state:** `recognize` CLI command works, using cosine distance (`<=>`) with a hardcoded threshold of `0.4`.

**Tasks**
- [x] Nearest-neighbor lookup with pgvector
- [x] Fall back to "Unknown" above the threshold
- [x] Return person ID
- [x] Pass the recognized ID profile to the LLM
- [x] Get a working output in the command line

**Definition of done:** Given a photo, the system returns a personalized response based on there profile.

---

## M3 — Display

**Goal:** Show the personalized info from the LLM on a display.

**Tasks**
- [x] Setup webapp basics
- [x] Uploading a photo of a person displays their personalized output
- [x] Guest view

**Definition of done:** Can upload an image to the site and it gives the personalized output, website does not need to be clean.

---

## M4 — Live Rendering

**Goal:** Display the output live based on who is in front of the camera.

**Tasks**
- [ ] Get access to the camera
- [ ] Update every 2-3 seconds (DeepFace is slow now)
- [ ] Recognize the person in front of the camera

**Definition of done:** Stepping in front of the camera shows the right person's display.

---

## M5 — Working Pipeline

**Goal:** Get a working pipeline of the app. 

**Tasks**
- [ ] Add register funcionality to the webapp

**Definition of done:** The app can be downloaded and ran end to end even if not pretty. 

---

---

## M6 — Clean Up

**Goal:** Speed the app up

**Tasks**

**Definition of done:** The app works as it should but much faster

---
## Open Questions

| # | Question | Milestone | Decision |
|---|----------|-----------|----------|

## Out of Scope

- Competing with commercial face-recognition products (this is a learning project).

## Changelog

- **2026-11-18** M3 (Display) and M4 (Live Rendering) marked done. Added M6
- **2026-11-16** M2 (Recognition) marked done.
- **2026-11-15:** Combined M2 and M3 and added more milestones.
- **2026-11-15:** M1 (Register) marked done.
- **2026-11-12:** Converted `plan.md` into this roadmap.
