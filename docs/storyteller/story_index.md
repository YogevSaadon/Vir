# Story Index

All stories in the game, organized by act. Status tracks the writing pipeline.

**Status key:** STUB → OUTLINE → DRAFT → REVIEW → DONE

---

## Act 1 — Arrival

| ID | Title | Status | Chain | Summary |
|----|-------|--------|-------|---------|
| `intro_god` | Voice of a God | DONE | — | God pulls you from the void, casts you into Vassnian |
| `mvp_story_01` | The Forest Path | DONE | — | Wake in forest, meet Aldric, goblin ambush, reach village |
| `mvp_story_02_shop` | The Village Market | DONE | — | Quiet village, meet merchant, buy supplies |

## Act 2 — Growth

(empty — waiting for content)

## Act 3 — Shadow World

(empty — waiting for content)

## Act 4 — Revenge & Final Battle

(empty — waiting for content)

---

## Story Chains

Chains link multi-part stories across acts.

| Chain ID | Stories | Acts | Status |
|----------|---------|------|--------|
| (none yet) | | | |

---

## How to Add a Story

1. Add a row to this index with status STUB and a 1-3 sentence summary
2. Create outline (node map with choice branches)
3. Write the JSON draft in `data/stories/<story_id>.json`
4. Add to `load_embedded_data()` in `vassnian_content/src/loader.rs`
5. Add to `mission_catalog.json` if it's a selectable mission
6. Update this index to DONE
