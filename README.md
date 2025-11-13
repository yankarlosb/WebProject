# Balance de Carga Docente

Sistema web para la gestión y cálculo del balance de carga docente en la Facultad de Ciberseguridad. Esta aplicación moderniza y automatiza el proceso de distribución de horas lectivas entre asignaturas, facilitando la planificación académica.

## Características Principales

- **Dashboard Intuitivo**: Vista general con estadísticas de años académicos, asignaturas y balances guardados
- **Cálculo Automático**: Distribución inteligente de horas entre diferentes tipos de actividades docentes (C, CP, S, PL, TE, T, PP, EC, TC, EF)
- **Gestión de Asignaturas**: CRUD completo para administrar asignaturas con validación de datos
- **Sistema de Roles**: Control de acceso basado en roles (Admin, Leader, SubjectLeader, User)
- **Autenticación Segura**: JWT con cookies HttpOnly y validación de IP
- **Interfaz Moderna**: Diseño responsive con Vue 3 y TailwindCSS 4
- **Gestión de Usuarios**: Panel de administración para crear y gestionar usuarios del sistema

## Tecnologías Utilizadas

### Backend
- **Rust** - Lenguaje de programación principal
- **Rocket 0.5.1** - Framework web asíncrono
- **SeaORM 1.1.17** - ORM para PostgreSQL
- **PostgreSQL** - Base de datos relacional
- **JWT (jsonwebtoken 9.3)** - Autenticación y autorización
- **bcrypt** - Hash seguro de contraseñas
- **Tokio** - Runtime asíncrono

### Frontend
- **Vue 3** - Framework JavaScript progresivo
- **TypeScript** - Tipado estático
- **Vite** - Build tool y dev server
- **Pinia** - State management
- **Vue Router** - Navegación SPA
- **TailwindCSS 4** - Framework CSS utility-first

## Requisitos Previos

- **Rust** (1.91.0 o superior)
- **Node.js** (18.x o superior)
- **PostgreSQL** (12 o superior)
- **npm** o **yarn**

## Instalación y Configuración

### 1. Clonar el Repositorio

```bash
git clone https://github.com/yankarlosb/WebProject.git
cd WebProject
```

### 2. Configurar la Base de Datos

```bash
# Crear base de datos PostgreSQL
createdb balance_carga

# Aplicar el esquema
psql -U postgres -d balance_carga -f backend/schema.sql
```

### 3. Configurar Variables de Entorno

Crear un archivo `.env` en la raíz del proyecto:

```env
DATABASE_URL=postgres://usuario:contraseña@localhost/balance_carga
JWT_SECRET=tu_clave_secreta_aqui_minimo_32_caracteres
```

### 4. Instalar y Ejecutar el Backend

```bash
cd backend

# Instalar dependencias (se hace automáticamente con cargo)
# Ejecutar en modo desarrollo
cargo run

# O compilar para producción
cargo build --release
./target/release/web_project
```

El backend estará disponible en `http://localhost:8000`

### 5. Instalar y Ejecutar el Frontend

```bash
cd frontend

# Instalar dependencias
npm install

# Ejecutar en modo desarrollo
npm run dev

# O compilar para producción
npm run build
```

El frontend estará disponible en `http://localhost:5173` (desarrollo)

## Esquema de Base de Datos

### Tabla `usuarios`
```sql
- id (SERIAL PRIMARY KEY)
- name (TEXT)
- email (TEXT UNIQUE)
- token (TEXT) -- Contraseña hasheada
- created_at (TIMESTAMP)
- role (TEXT) -- admin, leader, subjectLeader, user
```

### Tabla `asignaturas`
```sql
- id (SERIAL PRIMARY KEY)
- leader_id (INTEGER FK → usuarios.id)
- name (TEXT)
- year (TEXT)
- semester (TEXT)
- C, CP, S, PL, TE, T, PP, EC, TC, EF (INTEGER) -- Tipos de horas
- hours (INTEGER) -- Total de horas
- date_start, date_end (TIMESTAMP)
```

## Sistema de Autenticación

### Roles Disponibles
- **admin**: Acceso completo al sistema, gestión de usuarios
- **leader**: Gestión de asignaturas y balances
- **subjectLeader**: Gestión de asignaturas específicas
- **user**: Consulta de información

### Flujo de Autenticación
1. Usuario envía credenciales a `/api/login`
2. Backend valida y genera JWT
3. JWT se almacena en cookie HttpOnly
4. Todas las rutas protegidas validan el JWT automáticamente
5. Frontend mantiene estado de sesión en Pinia store

## API Endpoints

### Autenticación
- `POST /api/login` - Iniciar sesión
- `POST /api/logout` - Cerrar sesión
- `GET /api/verify` - Verificar sesión actual

### Gestión de Usuarios (Admin)
- `GET /api/list_users` - Listar usuarios
- `POST /api/create_user` - Crear usuario
- `POST /api/modify_user/:id` - Modificar usuario
- `POST /api/delete_user/:id` - Eliminar usuario

## Arquitectura del Proyecto

```
WebProject/
├── backend/
│   ├── src/
│   │   ├── database/          # Entidades SeaORM
│   │   ├── routes/            # Endpoints API
│   │   ├── utils/             # Utilidades (JWT, DB, CORS)
│   │   ├── types.rs           # Tipos compartidos
│   │   ├── lib.rs             # Configuración principal
│   │   └── main.rs            # Punto de entrada
│   ├── Cargo.toml
│   └── schema.sql
│
├── frontend/
│   ├── src/
│   │   ├── components/        # Componentes Vue reutilizables
│   │   ├── views/             # Páginas de la aplicación
│   │   ├── stores/            # Pinia stores
│   │   ├── services/          # Servicios API
│   │   ├── composables/       # Composables Vue
│   │   ├── router/            # Configuración de rutas
│   │   └── config/            # Configuración
│   ├── package.json
│   └── vite.config.ts
│
└── README.md
```

## Desarrollo

### Ejecutar Ambos Servicios

**Terminal 1 - Backend:**
```bash
cd backend
cargo run
```

**Terminal 2 - Frontend:**
```bash
cd frontend
npm run dev
```

### Compilar para Producción

**Backend:**
```bash
cd backend
cargo build --release
```

**Frontend:**
```bash
cd frontend
npm run build
```

Los archivos compilados del frontend estarán en `frontend/dist/` y el backend los servirá automáticamente en producción.

## Convenciones de Código

### Backend (Rust)
- Request guards para proteger rutas (`AuthenticatedUser`, `AdminUser`, etc.)
- Responses estandarizadas con `ApiResponse` y `ApiResponseWithData`
- Separación de lógica de negocio en `utils/db.rs`

### Frontend (Vue/TypeScript)
- Composables para lógica reutilizable
- Pinia stores para estado global
- Componentes con prefijo `App` para elementos base
- TypeScript strict mode habilitado

## 🔧 Configuración Adicional

### CORS
El backend permite solicitudes desde `http://localhost:5173` en desarrollo. Para producción, actualizar en `backend/src/utils/cors.rs`.

### Proxy de Desarrollo
El frontend en desarrollo usa proxy de Vite para redirigir `/api/*` al backend en `localhost:8000`.

## 🤝 Contribuciones

Este proyecto está en desarrollo activo. Para contribuir:

1. Fork el proyecto
2. Crea una rama para tu feature (`git checkout -b feature/nueva-funcionalidad`)
3. Commit tus cambios (`git commit -m 'Añadir nueva funcionalidad'`)
4. Push a la rama (`git push origin feature/nueva-funcionalidad`)
5. Abre un Pull Request

## Licencia

Este proyecto es de código abierto y está disponible bajo la licencia MIT.

## 👥 Autoresd
