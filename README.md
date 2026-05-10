# Sharplines
Open source, self-hostable chess preparation platform for serious players. Unlike Lichess or Chess.com, Sharplines is built exclusively for study and training and not playing.
## Tech Stack
- **Frontend:** SvelteKit + TypeScript + Tailwind CSS + Skeleton UI
- **Backend:** Rust (Axum) + SQLx
- **Database:** PostgreSQL 16 (Neon)
- **Auth:** Argon2id + JWT (access + refresh tokens) + httpOnly cookies
- **Email:** SMTP (Mailtrap for dev, any SMTP provider for production)
- **OAuth:** Lichess login with PKCE
## Features
- [x] Email/password registration with email verification
- [x] Lichess OAuth login
- [x] JWT with refresh token rotation
- [x] Password reset via email
- [ ] Study management (coming soon)
- [ ] Move tree with variations (coming soon)
- [ ] Training mode (coming soon)
- [ ] Stockfish WASM analysis (coming soon)
- [ ] LLM position analysis (coming soon)


## Quick Start
### Prerequisites
- Rust 2024 edition
- Node.js / Bun
- PostgreSQL 16 (local or [Neon](https://neon.tech))


### 1. Clone and set up
```bash
git clone https://github.com/MrSheerluck/sharplines.git
cd sharplines
```
2. Database
Create a PostgreSQL database (or use Neon):

Copy .env.example to backend/.env and set DATABASE_URL:
DATABASE_URL=postgres://user:pass@localhost:5432/sharplines

Run migrations:
```bash
cd backend
cargo install sqlx-cli
sqlx migrate run
```

3. Backend
```bash
cd backend
cargo run
```
The API server starts at http://localhost:3000.

4. Frontend
```bash
cd frontend
bun install
bun run dev
```
The app opens at http://localhost:5173.

5. Email (optional)
Set SMTP credentials in backend/.env. For development, use Mailtrap (https://mailtrap.io):
```bash
SMTP_HOST=sandbox.smtp.mailtrap.io
SMTP_PORT=2525
SMTP_USERNAME=your_username
SMTP_PASSWORD=your_password
```

### Using Neon
Sharplines works seamlessly with Neon's serverless Postgres (https://neon.tech):
1. Create a free Neon project
2. Copy your connection string from the Neon dashboard
3. Set it as DATABASE_URL in backend/.env
4. Run migrations: sqlx migrate run


Neon's free tier includes 0.5 GB storage and 100 compute hours/month, plenty for a personal instance.

## License
MIT
