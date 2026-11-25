# Facebase Roadmap

_Last updated: 2026-11-25_

## Vision

Recognize people in front of a camera and show each of them a personalized display (weather, to-dos, calendar, email, etc.). Unknown faces get a generic guest view.

## Status at a Glance

| # | Milestone | Status | Owner |
|---|-----------|--------|-------|
| M1 | Register | 🟢 Done | Hunter |
| M2 | Recognition | 🟢 Done | Hunter |
| M3 | Display | 🟢 Done | Hunter |
| M4 | Live Rendering | 🟢 Done | Hunter |
| M5 | Working Pipeline | 🟢 Done | Hunter |
| M6 | Clean Up | ⚪ Not Started | Hunter | 

Legend: ⚪ Not started · 🟡 In progress · 🟢 Done · 🔴 Blocked

---

## M1 — Register

**Goal:** Store a person's face embeddings in the database, linked to their profile.

**Current state:** Registration works from the CLI and the web app. It embeds a photo (ArcFace via DeepFace), inserts the person into `people`, and inserts the embedding into `face_embeddings`. Registration photos are deleted afterward (moved to M5).

**Tasks**
- [x] CLI `register` command with `--name`
- [x] Store embeddings in Postgres/pgvector
- [x] Handle photos with zero or multiple faces (uses the first face with confidence ≥ 0.8)
- ~~Make register photos temporary~~ → moved to M5
- ~~Register from multiple photos per person~~ → moved to M6

**Definition of done:** A person can be registered from a photo and stored with their embedding.

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
- [x] Get access to the camera
- [x] Update every 2-3 seconds (DeepFace is slow now)
- [x] Recognize the person in front of the camera

**Definition of done:** Stepping in front of the camera shows the right person's display.

---

## M5 — Working Pipeline

**Goal:** Get a working pipeline of the app. 

**Current state:** The app runs end to end with `docker compose up`: Postgres + pgvector and the app (Rust server + Python/DeepFace) each run in their own container. Tested from a fresh clone.

**Tasks**
- [x] Add register functionality to the webapp (#20)
- [x] Make register photos temporary and delete them after registration (#21)
- [x] Database setup script, easy database setup (`db/schema.sql`, #22)
- [x] Configure environment variables, not hardcoded (`DATABASE_URL`, `BIND_ADDR`, `PYTHON_DIR`, #23)
- [x] Add a .env example (#24)
- [x] Docker setup (`Dockerfile`, `docker-compose.yml`, #25)

**Definition of done:** The app can be downloaded and ran end to end even if not pretty.

---

## M6 — Clean Up

**Goal:** Speed the app up and make it nicer

**Tasks**
- [ ] Speed up embeddings (still undecided how)
- [ ] Make the match threshold configurable, for now it's hardcoded
- [ ] Register multiple photos per person to help with recognition, for now this is an error
- [ ] Clearer error messages
- [ ] Define the profile

**Definition of done:** The app works as it should but much faster and prettier

---
## Open Questions

| # | Question | Milestone | Decision |
|---|----------|-----------|----------|

## Out of Scope

- Competing with commercial face-recognition products (this is a learning project).

## Changelog

- **2026-11-25:** M5 (Working Pipeline) marked done. M1 updated to match the code. Added M6 tasks.
- **2026-11-24** M1 and M5 updated (moved a task from M1 -> M5)
- **2026-11-18** M3 (Display) and M4 (Live Rendering) marked done. Added M6
- **2026-11-16** M2 (Recognition) marked done.
- **2026-11-15:** Combined M2 and M3 and added more milestones.
- **2026-11-15:** M1 (Register) marked done.
- **2026-11-12:** Converted `plan.md` into this roadmap.
