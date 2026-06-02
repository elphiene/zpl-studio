
## 1. Identity

- **Name:** `zpl-tools`
- **One-liner:** `printer tools for zebra label printers - repo fork to include new features inspired by dymo label software.`
- **Why now:** `increase zpl tool ability inspired by the range of p[ossibilities on dymo.`

## 2. Account & repo

- **GitHub account:**
  - [x] `elphiene` — personal projects (default)
  - [ ] `rgb-b` — work / shop tools
- **Repo name:** `[zpl-tools-(something here to identify fork)`
- **Visibility:**
  - [x] Public — strip CLAUDE.md/.claude/PLAN.md before commits
  - [ ] Private
- **License:**
  - [ ] MIT
  - [ ] AGPL
  - [ ] Proprietary (work tool)
  - [x] Unlicensed — TBD

## 3. Audience & purpose

- **Primary user:** `[cherry + me, anyone who wants to use it, zebra label printer specific.`
- **Problem it solves:** `allows user to customise labels to a more extreme standard. previous version of zpl tools involes needing to understand some zpl code, and very minimal design options for labels. gives users templates, text control, image control, actual design options for fun and in built graphics libraries of clipart.`
- **"Done for now" means:** `user can add text and images to label and edit them like they would in a graphic editing software, like illustrator.`
- **Success signal:** `any user can easily create a custom label and save a template. workflow: user opens program, selects or adds label size dimensions. user adds text, changes font, size, style. user moves text and can align it with the label., user adds an image and can adjust the dot gain curve for the image, or user adds clipart from clipart library. user adjust size and position. user saves label and it can be reopened and reprinted whenever. user creates label template with variable data entry points (maybe name and address), user imports their data and has the data mapped how they want, user presses print and the amount of labels they need for their data automatically prints using their template. `

## 4. Tech stack ideas

Mark what you're **considering**. Don't lock in until you start coding.

### Frontend
- [ ] Pure HTML/CSS/JS (static site pattern — like `rgb-b`, `elphiene`)
- [ ] Vite + vanilla JS (app pattern — like `brandpack-tools`, `colour-match`)
- [ ] React + Vite + Tailwind (heavy app — like `whereis`)
- [ ] TUI (Python Textual or Rust ratatui — like `tidal-sweep`, `signal-tui`)
- [ ] N/A — backend only
- [x] Other: `rust + kotlin, any other necessary stacks`

### Backend
- [ ] Express + Node v24 ESM (like `brandpack-tools`, `colour-match`)
- [ ] Bun (like `whereis` backend)
- [x] Rust + axum (like `xrite-export`)
- [ ] Python (Flask/FastAPI/Textual)
- [ ] Cloudflare Worker (edge — like `elphiene-guestbook`)
- [ ] N/A — static / client-only
- [x] Other: `check repo`

### Database
- [x] SQLite (default — WAL + FKs, like everything else)
- [ ] D1 (only if Worker)
- [ ] None — in-memory / filesystem
- [ ] Other: `check repo`

### Build / bundler
- [ ] Vite
- [ ] Cargo
- [ ] None (vanilla static)
- [ ] Other: `check repo`

### Hard constraints (things you can't change)
- `[FILL IN — e.g. "must run on Node v24" / "must work offline" / "must talk to Tidal API" / "must use Cherry's existing colours"]`

### Influences (look at these projects for patterns)
- `always check back and reference github.com/cherrygaysoda/zebraprintlabel for development and tech stack, check cherrygaysoda github for other references to ideal stacks and design. `

## 5. Deployment

- **Where it runs:**
  - [ ] This machine via systemd (Linux service)
  - [ ] This machine via Docker
  - [ ] Cloudflare Pages (static)
  - [ ] Cloudflare Worker
  - [x] Local CLI only
  - [ ] Other: `[FILL IN]`
- **Domain:** `none; local program`
- **Cloudflare account that owns the zone:**
  - [ ] El's (`elphiene.com`, `rgb-b.com` — direct access)
  - [ ] Cherry's (`cherrysofa.com`, `cherryslabs.com` — needs her involvement)
  - [x] N/A — no domain
- **Port (if backend):** `[FILL IN — check `check-projects` for taken ports. Currently taken: 3000, 3030, 8080, 8081, 8082, 8083, 8096, 8181, 8920, 9090, 9115]`

## 6. Scope

### In scope for v0.1.0
- `text editing, align tools, clip art, images, sizing, templates`

### Out of scope (YAGNI — explicitly not doing these)
- `no mobile app or apk`

### Open questions (must answer before v0.1.0)
- `[FILL IN]`

## 7. Things to NOT do

Lessons from past projects, reminders to self.

- `[FILL IN — e.g. "don't introduce a framework before there's pain", "don't put secrets in URL params", "don't use better-sqlite3 on Node v24"]`

## 8. Visual / design notes

> Even if the project is "just" a tool, capture the aesthetic intent now — it shapes early decisions (font choice, layout density, dark vs light, icon style).

- **Vibe / mood:** `cherrys labs pallete, simple rust gui, user friendly`
- **Palette ideas:** `cherrys labs`
- **Type:** `rust gui`
- **Reference projects / sites:** `zpl tools, xrite-export local exe program gui`
- **Dark / light / both:** `dark`
- **Density:** `breathing room`
- **Logo / favicon:** `no logo yet, use zebra label printer logo`

## 9. Notes for Claude

Anything specific to keep in mind while working on this.

- `this is a repo originally created by cherry. i want to make a fork of it to add some features. do not rework the exisiting structure unless necessary`

## 10. First milestones

Rough order of operations. Numbered so it's easy to talk about ("let's tackle 2 next").

1. `claude.md and scaffolding, repo analysis`
2. `visual wireframe, visual and tech stack suggestions`
3. `check in with cherry`
4. `build first demo`

---

## Initial setup checklist (after filling the above)

- [x] Create folder under `~/Documents/El-Projects/<name>/`
- [ ] `git init`
- [ ] Standard layout: `README.md`, `CLAUDE.md`, `.gitignore`, `docs/`, `deploy/` (if applicable)
- [ ] `.gitignore` includes `CLAUDE.md`, `.claude/`, `PLAN.md`, plus stack-specific entries (`node_modules/`, `target/`, `.env`, etc.)
- [ ] `gh repo create <account>/<name> --public/--private`
- [ ] First commit → tag `v0.0.1`
- [ ] `gh auth switch --user <account>` — verify you're on the right account before push
- [ ] If domain involved: add ingress to `~/.cloudflared/config.yml`, then DNS (whichever Cloudflare account)
- [ ] If service involved: write `deploy/<name>.service`, symlink to `/etc/systemd/system/`
- [ ] Add a healthcheck entry to `~/.local/bin/check-projects`
- [ ] Distil sections 1, 4, 5 of this brief into the new `CLAUDE.md`'s status block
