# Copilot Instructions: Balance de Carga Docente

## Project Architecture

**Stack**: Rust Rocket backend + Vue 3 + TypeScript frontend with PostgreSQL
- **Backend**: Rocket 0.5.1 + SeaORM 1.1.17 (PostgreSQL ORM) on port 8000
- **Frontend**: Vue 3 + Vite + Pinia + TailwindCSS 4 on port 5173 (dev)
- **Database**: PostgreSQL with `usuarios` and `asignaturas` tables

### Key Architectural Patterns

**Monorepo Structure**: Backend serves frontend static files in production. In dev, Vite proxies `/api/*` to `localhost:8000`.

**Authentication Flow**: HttpOnly JWT cookies (not localStorage tokens)
- Backend sets `jwt_token` cookie (HttpOnly, SameSite=Lax, Secure)
- Frontend stores minimal auth state (`loggedIn`, `currentUser`) for UX
- All protected routes verified via `AuthenticatedUser` request guard on backend
- Router guard calls `AuthService.checkAuth()` which validates JWT server-side

**Database Pattern**: SeaORM entities auto-generated from schema
- Entities in `backend/src/database/{usuarios,asignaturas}.rs`
- Relations: `usuarios` has_one `asignaturas` via `jefe_id`
- Regenerate entities: Use `sea-orm-cli` after schema changes

## Critical Conventions

### Backend (Rust/Rocket)

**Request Guards**: Use `AuthenticatedUser(Claims)` or `User(Claims)` for protected routes
```rust
#[get("/balance")]
pub async fn balance_page(_user: AuthenticatedUser) -> Option<NamedFile>
```

**JWT Management**: `utils/jwt.rs` handles creation/validation
- Secret from `.env` `JWT_SECRET` (required)
- 24h token expiration default
- Cookie extraction in guard, NOT from Authorization header

**CORS**: Fairing in `utils/cors.rs` allows `localhost:5173` with credentials
- Update `Access-Control-Allow-Origin` for production

**Database Connection**: `utils/db::establish_connection()` uses `DATABASE_URL` from `.env`
- Connection pooled via Rocket's `State<AppState>`

**Route Structure**:
- `routes/login.rs`: Auth endpoints (`/api/login`, `/api/logout`, `/api/verify`)
- `routes/manager.rs`: User management
- All API routes mounted under `/api` prefix

### Frontend (Vue 3/TypeScript)

**Auth Service Pattern**: `services/auth.ts` is the single source of truth
- `login()`: POST to `/api/login`, stores user in localStorage for UX
- `checkAuth()`: GET to `/api/verify`, syncs with backend JWT state
- `logout()`: POST to `/api/logout`, clears local state
- Never manually manage cookies (backend-controlled)

**Router Guards**: `router/index.ts` beforeEach hook
1. Check `requiresAuth` meta
2. Quick check `isLocallyAuthenticated()` (localStorage flag)
3. Call `checkAuth()` to validate with backend
4. Redirect to `/login` if JWT invalid

**API Configuration**: `config/api.ts` centralizes endpoints
- Dev: Empty `BASE_URL` (Vite proxy handles routing)
- Prod: Backend serves from same origin

**Composable Pattern**: Use `composables/useAuth.ts` in components
```typescript
const { user, isAuthenticated, logout } = useAuth()
```

**Component Structure**:
- `views/`: Page components (Login, Dashboard, BalanceForm, Configuracion, Asignaturas)
- `components/`: Reusable UI (BaseButton, Toast, ConfirmModal)
- `.backup` files: Previous versions kept for reference (cleanup when stable)

## Development Workflows

### Starting Development
```fish
# Terminal 1: Backend (requires PostgreSQL running + .env with DATABASE_URL, JWT_SECRET)
cd backend
cargo run

# Terminal 2: Frontend
cd frontend
npm run dev
```

**Environment Setup**: `.env` in project root (NOT in backend/)
```env
DATABASE_URL=postgres://user:pass@localhost/dbname
JWT_SECRET=your-secret-key-here
```

### Database Management

**Apply Schema**: Run `backend/schema.sql` manually on PostgreSQL
```fish
psql -U user -d dbname -f backend/schema.sql
```

**Regenerate Entities** (after schema changes):
```fish
cd backend
sea-orm-cli generate entity -o src/database --with-serde both
```

**Password Hashing**: Uses bcrypt cost 12
- Test hash printed on backend startup: `admin token: $2b$12$...`
- Insert users with pre-hashed passwords in SQL or via `/api/manager` endpoint

### Building for Production

**Frontend**:
```fish
cd frontend
npm run build  # Output: frontend/dist/
```

**Backend**: Serves `frontend/dist/` when not in debug mode
```fish
cd backend
cargo build --release
```

**Deployment Note**: Ensure `JWT_SECRET` in production `.env` and update CORS origin

## Common Pitfalls

1. **Cookie Auth Confusion**: Frontend should NEVER read/write `jwt_token` cookie directly. Backend manages it.

2. **Database Entity Modifications**: Don't manually edit `database/{usuarios,asignaturas}.rs` - they're auto-generated. Update `schema.sql` and regenerate.

3. **CORS in Production**: Update `utils/cors.rs` to allow production domain, not `localhost:5173`.

4. **Role Field**: User roles stored as `role` (String) in DB, validated in JWT guard. Values: `"admin"`, `"user"`, `"leader"`, `"subjectLeader"`.

5. **API Proxy**: Vite dev server proxies `/api/*` to backend. In production, Rocket handles all routes (API + static files).

6. **Environment Variables**: Backend expects `.env` in project root (parent of `backend/` and `frontend/`), NOT in `backend/.env`.

## Testing User Scenarios

**Create Test User** (via psql):
```sql
-- Password will be 'admin'
INSERT INTO usuarios (name, email, token, role) VALUES 
  ('Admin User', 'admin@test.com', '$2b$12$hash_here', 'admin');
```

**Test Auth Flow**:
1. Open `http://localhost:5173/login`
2. Login creates cookie, redirects to `/dashboard`
3. Refresh page - router guard validates JWT
4. Logout clears cookie, redirects to `/login`
