# Contributing to Sharplines

## Getting Started

1. Fork the repo
2. Create a branch: `git checkout -b feat/your-feature`
3. Follow the [README](README.md) quick start to set up locally
4. Make your changes
5. Run tests: `cd frontend && bun run test` and `cd backend && cargo test`
6. Push and open a Pull Request

## Development Setup

### Environment Variables

Copy `backend/.env.example` to `backend/.env` and fill in the values. For local development, most features work with dummy values except `DATABASE_URL`.

### Database

You can use either local PostgreSQL or [Neon](https://neon.tech). The free tier of Neon is sufficient.

```bash
# Local
createdb sharplines

# Or set Neon URL in .env
```

Run migrations:

```bash
cd backend && sqlx migrate run
```

### Email

For email testing, create a free [Mailtrap](https://mailtrap.io) account and use their SMTP credentials. Emails will appear in your Mailtrap inbox instead of being sent to real addresses.

## Project Structure

```
backend/
├── migrations/          # SQL migrations
├── src/
│   ├── auth/           # Auth modules (password, jwt, middleware, lichess)
│   ├── email/          # SMTP email sending
│   ├── handlers/       # Route handlers (auth.rs)
│   ├── models/         # Database models and DTOs
│   ├── config.rs       # Environment config
│   ├── db.rs           # Database pool
│   ├── errors.rs       # Error types
│   └── main.rs         # Server startup, router, CORS

frontend/
├── src/
│   ├── lib/
│   │   ├── api/        # API client (fetch wrapper, token refresh)
│   │   ├── assets/     # Static assets
│   │   ├── components/ # UI components
│   │   └── stores/     # Svelte 5 reactive stores
│   └── routes/         # SvelteKit pages
│       ├── login/
│       ├── register/
│       ├── verify-email/
│       ├── forgot-password/
│       └── reset-password/
```

## Code Style

### Backend (Rust)

- Follow `cargo clippy` suggestions
- Use `sqlx::query_as::<_, T>` (runtime queries) unless compile-time checking is preferred
- Error handling: return `impl IntoResponse` with appropriate status codes
- Tests: `#[cfg(test)] mod tests` inline in source files

### Frontend (SvelteKit)

- Use Svelte 5 runes (`$state`, `$derived`, `$effect`)
- Component tests alongside components (`+page.svelte` → `page.test.ts`)
- Use Tailwind utility classes for styling
- Import via `$lib/` alias (not relative paths)

## Testing

```bash
# Frontend
cd frontend && bun run test

# Backend (unit tests only, no database needed)
cd backend && cargo test
```

Integration tests require a running database and are not yet implemented.

## Pull Request Guidelines

- One feature per PR
- Include tests for new functionality
- Update the README if adding dependencies or setup steps
- Keep the frontend in sync with any backend API changes
- AI Usage is allowed as long as you understand what you did and why. Also please write PR on your own, don't use AI for that
- If you are in doubt, do not hesitate to reach out to me

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
