# Copilot Instructions: Balance de Carga Docente

Sistema web para gestión de carga docente en Facultad de Ciberseguridad - Rust Rocket + Vue 3 + PostgreSQL monorepo.

## Architecture Overview

**Stack**: Rust backend (Rocket 0.5.1 + SeaORM 1.1.17) + Vue 3/TypeScript frontend + PostgreSQL
- Backend: Port 8000, serves API (`/api/*`) and static files in production
- Frontend: Port 5173 (dev), Vite proxies `/api/*` to backend
- Auth: HttpOnly JWT cookies with IP validation (3h expiration)
- State: Pinia stores (`auth`, `balance`, `asignaturas`, `users`)

**Data Flow Pattern**: Frontend Pinia store → Service layer (`services/*.ts`) → Backend API (`routes/*.rs`) → SeaORM → PostgreSQL

**Critical Design Decision**: Balance calculations stored **client-side only** (localStorage). `backend/src/database/balance.rs` entity exists but balances are NOT persisted to DB - only `asignaturas` and `usuarios` tables are active. Balance data uses 79-cell grid (15 weeks × 4 days + consultation week + exams).

## Database Schema

**3 Tables** (`backend/schema.sql`):
- `usuarios`: User accounts with bcrypt tokens. Fields: `id`, `user_name`, `name`, `email`, `token`, `role`, `created_at`
- `asignaturas`: Subjects with hourly distribution (C, CP, S, PL, TE, T, PP, EC, TC, EF columns). Has `leader_id` FK to `usuarios.id`
- `schema_migrations`: Version tracking (migrations directory mentioned but not implemented)

**SeaORM Entities**: Auto-generated in `backend/src/database/`, **never edit manually**. Regenerate after schema changes:
```fish
cd backend
sea-orm-cli generate entity -o src/database --with-serde both
```

## Authentication System

**JWT Flow** (IP-bound tokens):
1. `POST /api/login` validates credentials → generates JWT with user IP → sets HttpOnly cookie (`jwt_token`)
2. All protected routes use request guards: `AuthenticatedUser`, `AdminUser`, `LeaderUser`, etc. (defined in `utils/jwt.rs`)
3. Frontend stores user metadata in localStorage for UX **only** - cookie holds real auth state
4. `GET /api/verify` validates JWT server-side (called by router guard on navigation)

**Role Hierarchy**: `admin` → `leader` → `subjectLeader` → `user`
- Admin: User CRUD via `/api/create_user`, `/api/delete_user/<id>`, `/api/modify_user/<id>`, `/api/list_users`
- Guard composition example: `AdminUser(Claims)` wraps `AuthenticatedUser` with role check

**Security Notes**:
- JWT secret from `.env` `JWT_SECRET` (required, loaded via `once_cell::sync::Lazy`)
- Cookie flags: `http_only=true`, `same_site=Lax`, `secure=true`, `max_age=3h` (10800s)
- IP validation in `decode_jwt()` compares `claims.ip` with `request.remote()` address
- Passwords hashed with bcrypt (cost=12)

## Frontend Patterns

**Router Guard Logic** (`router/index.ts`):
1. Check route `meta.requiresAuth` and `meta.requiresAdmin`
2. Fast-fail: Check `authStore.isAuthenticated` (localStorage flag)
3. Backend verify: Call `authStore.checkAuth()` → hits `/api/verify`
4. On 401: Clear store, redirect to `/login`

**State Management** (Pinia stores):
- `auth.ts`: User session, `checkAuth()`, `logout()`, `isAdmin` getter
- `balance.ts`: Client-side balance CRUD in localStorage (keys: `balance_<id>`, `recentBalances`)
- `users.ts`: Admin user management, mirrors backend `/api/list_users`

**Component Naming**: 
- Base components: `App*` prefix (`AppButton`, `AppModal`, `AppInput`)
- Feature components: Domain-specific (`BalanceWeekTable`, `StatsCard`)

**TypeScript Types**: Shared via `services/*.ts` (e.g., `AuthResponse`, `User` in `auth.ts`)

## Development Workflows

**Setup** (one-time):
```fish
# Root .env file (NOT in backend/)
echo 'DATABASE_URL=postgres://user:pass@localhost/balance_carga' > .env
echo 'JWT_SECRET=your-32-char-minimum-secret-key' >> .env

# Create DB and apply schema
createdb balance_carga
psql -U postgres -d balance_carga -f backend/schema.sql
```

**Daily Dev** (2 terminals):
```fish
# Terminal 1: Backend (prints bcrypt hash of "admin" on startup)
cd backend && cargo run

# Terminal 2: Frontend
cd frontend && npm run dev
```

**Production Build**:
```fish
cd frontend && npm run build              # → frontend/dist/
cd ../backend && cargo build --release    # Serves dist/ when cfg!(not(debug_assertions))
```

**Add Test User** (password "admin"):
```fish
# Get hash from backend startup output or generate:
cargo run  # Copy printed hash: admin token: $2b$12$...
psql -d balance_carga -c "INSERT INTO usuarios (user_name, name, email, token, role) VALUES ('admin', 'Admin User', 'admin@test.com', '<hash_here>', 'admin');"
```

## API Conventions

**Response Types** (`types.rs`):
- `ApiResponse`: `{ message: String, alert: "success"|"error" }`
- `ApiResponseWithData<T>`: Adds `data: Option<T>` field

**Route Mounting** (`lib.rs`):
- All API routes under `/api` prefix via `rocket::mount("/api", routes![...])`
- Static files from `../frontend/src` (debug) or `../frontend/dist` (release)
- Catch unauthorized: `#[catch(401)]` catcher returns HTML alert + redirect to login

**CORS Config** (`utils/cors.rs`): 
- Dev: Allows `http://localhost:5173` with credentials
- **Action Required**: Update `Access-Control-Allow-Origin` for production domain

## Common Mistakes

1. **`.env` Location**: Must be in project root (sibling to `backend/` and `frontend/`), NOT `backend/.env`
2. **Balance Persistence**: Balances are localStorage-only - don't expect DB queries to return them
3. **Entity Editing**: Regenerate entities after schema changes, don't modify `database/*.rs` files
4. **Cookie Confusion**: Frontend NEVER reads/writes `jwt_token` cookie - only backend manages it via `CookieJar`
5. **Fish Shell Syntax**: Use `echo` or `printf`, NOT bash heredocs (`<<EOF`)
6. **User Table Column**: Uses `user_name` for login username, `name` for display name (easy typo)

## Key Files Reference

- **Backend Entry**: `backend/src/main.rs` (prints admin hash) → `lib.rs` (Rocket config)
- **Auth Implementation**: `backend/src/utils/jwt.rs` (guards + claims)
- **Schema Source**: `backend/schema.sql` (single source of truth)
- **Frontend Router**: `frontend/src/router/index.ts` (auth guard logic)
- **Auth Service**: `frontend/src/services/auth.ts` (login/logout/verify)
- **Vite Proxy**: `frontend/vite.config.ts` (`/api` → `localhost:8000`)

## Domain Logic

**Balance Calculation** (frontend-only):
- 79-cell grid per subject: 15 weeks × 4 days + consultation week + exams
- Coefficient formula: `total * 1.2` (see `stores/balance.ts` `calculateAll()`)
- Distribution types: C (Conference), CP (Practical Conference), S (Seminar), PL (Lab Practice), TE (Thesis), T (Tutorial), PP (Pre-Professional), EC/TC (Exam Committees), EF (Final Exam)

**Subject Hours**: Integer columns per activity type (`asignaturas` table), total stored in `hours` column

## Profile Management

**User Profile Update**: 
- Any authenticated user can update their own profile via `POST /api/update_profile` (name, email)
- Password change via `POST /api/change_password` (new_password)
- Both endpoints use `AuthenticatedUser` guard, extract user ID from JWT claims
- See `routes/manager.rs` for implementation details
