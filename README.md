# 🎄 Rust/Vue Christmas Calendar

An open-source learning project for building an **interactive Advent calendar** using **Rust (Backend)** and **Nuxt 3 (Frontend)**. The goal is to learn modern web development with a secure, high-performance stack — while creating something meaningful and fun to share.

---

## 🌟 Concept

1. **User** registers and creates a calendar.
2. **24 gifts** are assigned to the days.
3. **Invite** another person via a generated invitation link.
4. **Opening doors:** The invited person can open the door of the current day (or past days) only.

---

## 🧰 Tech Stack

| Layer          | Technology                                                        | Notes                                             |
| -------------- | ----------------------------------------------------------------- | ------------------------------------------------- |
| Frontend       | [Nuxt 3](https://nuxt.com) + [Nuxt UI](https://ui.nuxt.com)       | Mobile-first, SSR, Figma-based design system      |
| Backend        | [Rust](https://www.rust-lang.org/) + [Axum](https://docs.rs/axum) | API server, authentication, calendar logic        |
| Database       | PostgreSQL / SQLx                                                 | Local via Docker, or managed via Supabase/Hetzner |
| Infrastructure | Docker Compose, GitHub Actions, Vercel/Fly.io                     | CI/CD & deployment                                |
| Design         | Figma                                                             | Tokens & UI states                                |

---

## 🗂️ Project Structure (Monorepo Plan)

```
weihnachtskalender/
  api/           → Rust (Axum)
  web/           → Nuxt 3 + Nuxt UI
  db/            → SQL sketches & migrations
  docs/          → Architecture, API, Security
  .github/       → Actions, templates
  README.md
```

---

## 📅 Roadmap

| Milestone | Goal               | Scope                                       |
| --------- | ------------------ | ------------------------------------------- |
| **M0**    | Project foundation | Repo, structure, CI plan, documentation (✅) |
| **M1**    | Auth system        | Register/Login, sessions, `/me` endpoint    |
| **M2**    | Calendar logic     | CRUD + seed 24 default slots                |
| **M3**    | Gifts              | Upsert + lock days 6 & 24                   |
| **M4**    | Invitations        | Invite link + join flow                     |
| **M5**    | Door opening       | Gate logic & open tracking                  |
| **M6**    | Budget & UI        | Price totals & progress bar                 |
| **M7**    | Deployment         | Vercel (frontend), Fly/Hetzner (API + DB)   |

---

## 🧪 Local Setup (Pseudocode)

```bash
# 1. Clone repository
git clone https://github.com/<user>/weihnachtskalender.git
cd weihnachtskalender

# 2. Start dev environment
docker compose -f docker/compose.dev.yml up --build

# 3. Access:
# API → http://localhost:8080
# Web → http://localhost:3000
```

---

## 🔒 Security (Plan)

* Session cookies (HttpOnly, SameSite=Lax) or JWT (later stage).
* Rate-limiting on auth & door-opening routes.
* CSRF protection if using cookies.
* Timezone validation **server-side only**.

---

## 🤝 Contributing

This project is **open to contributions** — whether it’s code, design, testing, or documentation.

### Getting Started

1. Fork the repo.
2. Create a branch: `feat/<your-feature>`.
3. Commit using [Conventional Commits](https://www.conventionalcommits.org/).
4. Open a Pull Request.

### Community

* Use **GitHub Discussions** for ideas & questions.
* Helpful labels: `good first issue`, `help wanted`, `design`, `api`, `web`.

---

## 🧭 Vision

A project that:

* Is **fun** to build 👩‍💻
* **Spreads joy** through sharing 🎁
* Encourages learning (Rust, Nuxt, Docker, CI/CD) 📚
* Serves as an **open-source starter** for others 🌍

---

## 📜 License

This project is licensed under the **MIT License** (see LICENSE file).

---

> 💡 *“One door per day — one commit per week.”*
