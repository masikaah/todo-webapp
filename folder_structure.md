taskflow/
├── Cargo.toml                     # Workspace definition
├── Cargo.lock
├── .env.example                   # Environment variables template
├── .gitignore
├── README.md
├── scripts/                       # Build & deployment scripts
│   ├── dev.sh
│   └── deploy.sh
├── migrations/                    # Database migrations (e.g., SQLx)
│   ├── 20240301000000_init.sql
│   └── ...
├── docker/
│   ├── backend.Dockerfile
│   └── frontend.Dockerfile
├── shared/                        # Shared types, utilities, and protocols
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── models/                # Data structures used by both frontend & backend
│       │   ├── mod.rs
│       │   ├── task.rs
│       │   └── user.rs
│       ├── websocket/             # WebSocket message types
│       │   ├── mod.rs
│       │   └── messages.rs
│       └── utils/                 # Common utilities (e.g., id generation)
│           ├── mod.rs
│           └── id.rs
├── backend/                       # Rust backend server
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs                # Entry point
│       ├── lib.rs                 # (optional) for integration tests
│       ├── config.rs              # Configuration loading (env, settings)
│       ├── db/                    # Database layer
│       │   ├── mod.rs
│       │   ├── connection.rs
│       │   └── repositories/
│       │       ├── mod.rs
│       │       ├── task_repository.rs
│       │       └── user_repository.rs
│       ├── api/                   # HTTP / WebSocket handlers
│       │   ├── mod.rs
│       │   ├── http/              # REST endpoints
│       │   │   ├── mod.rs
│       │   │   ├── tasks.rs
│       │   │   └── health.rs
│       │   └── ws/                # WebSocket handlers
│       │       ├── mod.rs
│       │       ├── connection.rs
│       │       └── task_sync.rs
│       ├── services/              # Business logic
│       │   ├── mod.rs
│       │   ├── task_service.rs
│       │   └── sync_service.rs
│       ├── middleware/            # Custom middleware (auth, logging)
│       │   ├── mod.rs
│       │   ├── auth.rs
│       │   └── logger.rs
│       └── errors/                # Error types and conversions
│           ├── mod.rs
│           └── api_error.rs
├── frontend/                      # Rust frontend (Yew / Dioxus / Leptos)
│   ├── Cargo.toml
│   ├── index.html                 # Static entry point
│   ├── styles/                    # CSS / Tailwind (if used)
│   │   └── main.css
│   ├── public/                    # Static assets (images, icons)
│   │   └── favicon.ico
│   └── src/
│       ├── main.rs                # Entry point (or lib.rs + main.rs)
│       ├── app.rs                 # Root component
│       ├── components/            # Reusable UI components
│       │   ├── mod.rs
│       │   ├── task_item.rs
│       │   ├── task_list.rs
│       │   ├── add_task_form.rs
│       │   └── undo_snackbar.rs
│       ├── pages/                 # Page-level components
│       │   ├── mod.rs
│       │   ├── home.rs
│       │   └── settings.rs
│       ├── services/              # API client & WebSocket connection
│       │   ├── mod.rs
│       │   ├── api_client.rs      # HTTP client (e.g., reqwest)
│       │   └── websocket.rs       # WebSocket client
│       ├── store/                 # State management (e.g., Redux-like)
│       │   ├── mod.rs
│       │   ├── tasks.rs
│       │   └── sync.rs
│       ├── utils/                 # Frontend utilities
│       │   ├── mod.rs
│       │   └── optimistic.rs
│       └── cache/                 # Local storage (IndexedDB, localStorage)
│           ├── mod.rs
│           └── task_cache.rs
└── tests/                         # Integration tests
    ├── api_tests.rs
    └── ws_tests.rs