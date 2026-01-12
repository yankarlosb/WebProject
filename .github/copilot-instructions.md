# Copilot Instructions: Balance de Carga Docente

Sistema web para gestión de carga docente en Facultad de Ciberseguridad - Rust Rocket + Vue 3 + PostgreSQL monorepo.

## Architecture Overview

**Stack**: Rust backend (Rocket 0.5.1 + SeaORM 1.1.17) + Vue 3/TypeScript frontend + PostgreSQL
- Backend: Port 8000, serves API (`/api/*`) and static files in production
- Frontend: Port 5173 (dev), Vite proxies `/api/*` to backend
- Auth: HttpOnly JWT cookies with IP validation (3h expiration)
- State: Pinia stores (`auth`, `balance`, `asignaturas`, `users`, `ui`)

**Data Flow**: Pinia store → Service (`services/*.ts`) → HTTP helper (`services/http.ts`) → Backend route (`routes/*.rs`) → Business logic (`utils/db.rs`) → SeaORM → PostgreSQL

## Database Schema (5 Tables)

**Schema Source**: `backend/schema.sql` — **Migrations**: `backend/migrations/*.sql` (apply via psql)

| Table | Purpose |
|-------|---------|
| `usuarios` | User accounts with bcrypt tokens. Login uses `user_name`, display uses `name` |
| `asignaturas` | Subjects with hourly distribution (C, CP, S, PL, TE, T, PP, EC, TC, EF). `leader_id` is UNIQUE FK |
| `balances` | Balance metadata (academic_year, period, weeks, status, deadline, non_academic_periods JSONB) |
| `balance_fragments` | Per-asignatura data within a balance. Links to `asignatura_id` and `subject_leader_id` |
| `audit_logs` | Security/functional auditing (event_type, category, entity_type, success, ip_address) |

**SeaORM Entities**: Auto-generated in `backend/src/database/`. Regenerate after schema changes:
```fish
cd backend && sea-orm-cli generate entity -o src/database --with-serde both
```

## API Design Conventions (CRITICAL)

### RESTful Routes Pattern
All routes MUST follow RESTful conventions:
```
GET    /api/{resource}           # List all
POST   /api/{resource}           # Create new
GET    /api/{resource}/<id>      # Get one
PUT    /api/{resource}/<id>      # Update one
DELETE /api/{resource}/<id>      # Delete one
```

### Current API Endpoints
| Resource | Endpoints |
|----------|-----------|
| Auth | `POST /api/login`, `POST /api/logout`, `GET /api/verify` |
| Users | `GET /api/users`, `POST /api/users`, `PUT /api/users/<id>`, `DELETE /api/users/<id>` |
| Profile | `PUT /api/profile`, `PUT /api/profile/password` |
| Asignaturas | `GET /api/asignaturas`, `POST /api/asignaturas`, `PUT /api/asignaturas/<id>`, `DELETE /api/asignaturas/<id>` |
| Balances | `GET /api/balances`, `POST /api/balances`, `GET /api/balances/<id>`, `PUT /api/balances/<id>`, `DELETE /api/balances/<id>` |
| Fragments | `GET /api/fragments/pending`, `GET /api/balances/<id>/fragments/<asig_id>`, `PUT /api/balances/<id>/fragments/<asig_id>` |

### Response Types (`types.rs`)
- `ApiResponse`: `{ message, alert: "success"|"error" }`
- `ApiResponseWithData<T>`: Adds `data: Option<T>`

## Frontend Service Patterns (CRITICAL)

### Standard Service Pattern
All services MUST use object literal pattern with `ServiceResponse<T>`:
```typescript
// services/example.ts
import { httpGet, httpPost, httpPut, httpDelete, type ServiceResponse } from './http'

export const exampleService = {
  async list(): Promise<ServiceResponse<Item[]>> {
    return httpGet<Item[]>('/api/items', 'Error al obtener items')
  },
  async create(data: CreateData): Promise<ServiceResponse<void>> {
    return httpPost('/api/items', data, 'Error al crear item')
  },
  async update(id: number, data: UpdateData): Promise<ServiceResponse<void>> {
    return httpPut(`/api/items/${id}`, data, 'Error al actualizar item')
  },
  async delete(id: number): Promise<ServiceResponse<void>> {
    return httpDelete(`/api/items/${id}`, 'Error al eliminar item')
  },
}
export default exampleService
```

### Store Pattern
Stores use consistent action naming (`fetch*`, `create*`, `update*`, `delete*`):
```typescript
async function fetchItems() {
  isLoading.value = true
  error.value = null
  try {
    const response = await itemsService.list()
    if (response.success && response.data) {
      items.value = response.data
      return { success: true }
    }
    error.value = response.message || 'Error'
    return { success: false, message: error.value }
  } catch (err) {
    error.value = 'Error de conexión'
    return { success: false, message: error.value }
  } finally {
    isLoading.value = false
  }
}
```

## Authentication System

**JWT Flow** (IP-bound):
1. `POST /api/login` → JWT in HttpOnly cookie (`jwt_token`)
2. Request guards validate: `AuthenticatedUser`, `AdminUser`, `LeaderUser`, `SubjectLeaderUser`, `LeaderOrSubjectLeaderUser`
3. Frontend calls `authStore.checkAuth()` → `/api/verify` → user data in memory (NOT localStorage)

**Role Hierarchy**: `admin` > `leader` > `subjectLeader` > `user`
- Guards use `impl_role_guard!` macro in `utils/jwt.rs`

## Balance System Architecture

**Leader/SubjectLeader Workflow**:
1. **Leader** creates balance → selects asignaturas → fragments auto-created (one per asignatura)
2. **SubjectLeader** fills their assigned fragment (weekly distribution grid)
3. **Leader** views progress across all fragments, can set `deadline` and `allow_leader_edit`

**Fragment Status Flow**: `pending` → `in_progress` → `completed`

## Development Workflows

**Setup** (one-time):
```fish
# Root .env file (NOT backend/.env)
echo 'DATABASE_URL=postgres://user:pass@localhost/balance_carga' > .env
echo 'JWT_SECRET=your-32-char-minimum-secret-key' >> .env

createdb balance_carga
psql -U postgres -d balance_carga -f backend/schema.sql
```

**Daily Dev** (2 terminals):
```fish
cd backend && cargo run      # Terminal 1
cd frontend && npm run dev   # Terminal 2
```

## Common Mistakes

1. **`.env` Location**: Project root, NOT `backend/.env`
2. **Entity Editing**: Never edit `database/*.rs` — regenerate with sea-orm-cli
3. **Cookie Handling**: Frontend NEVER reads `jwt_token` cookie — backend manages via `CookieJar`
4. **Column Names**: `user_name` for login, `name` for display (easy typo)
5. **Fish Shell**: Use `echo`/`printf`, NOT bash heredocs
6. **Service Pattern**: Always use object literal + `ServiceResponse<T>`, NOT class with static methods
7. **HTTP Verbs**: Use proper verbs (GET/POST/PUT/DELETE), NOT `POST` for everything
8. **Route Naming**: RESTful `/resource/<id>`, NOT `/action_resource/<id>`

## Key Files Reference

| Purpose | File |
|---------|------|
| Backend entry | `backend/src/main.rs` → `lib.rs` |
| Auth guards | `backend/src/utils/jwt.rs` |
| DB operations | `backend/src/utils/db.rs` |
| Balance routes | `backend/src/routes/balance.rs` |
| User/Asignatura routes | `backend/src/routes/manager.rs` |
| Audit logging | `backend/src/utils/audit.rs` |
| Frontend router | `frontend/src/router/index.ts` |
| HTTP helpers | `frontend/src/services/http.ts` |
| Balance store | `frontend/src/stores/balance.ts` |
| API config | `frontend/src/config/api.ts` |
