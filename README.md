# API REST Rust — V2

Refactoring complet de l'API REST construite from scratch en Rust.  
L'objectif est de produire un code propre, testé, et prêt pour une connexion à une base de données.

---

## Objectifs pédagogiques

- Consolider les concepts vus en V1 : ownership, borrowing, enums, structs, traits, closures, Arc/Mutex
- Introduire les nouveaux concepts : error handling custom, tests, async/await, BDD

---

## Plan de développement

### Phase 1 — Architecture propre

Recréer le projet from scratch avec une structure de modules claire :

```
└── 📁src
    └── 📁domain
        └── 📁task
            ├── mod.rs
            ├── model.rs
            ├── repository.rs
            ├── service.rs
        ├── mod.rs
    └── 📁errors
        ├── mod.rs
    └── 📁infra
        └── 📁http
            └── 📁handlers
                ├── mod.rs
                ├── task_handler.rs
            ├── mod.rs
            ├── request.rs
            ├── response.rs
        └── 📁router
            ├── mod.rs
        ├── mod.rs
    └── main.rs
```

**Contraintes :**
- Pas de `unwrap()` en dehors des tests
- Chaque module a une responsabilité unique
- Les handlers ne connaissent pas les détails HTTP

---

### Phase 2 — Error handling propre

Créer un type d'erreur custom avec `thiserror` :

```rust
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Route not found")]
    NotFound,
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Internal server error")]
    Internal,
}
```

Tous les `unwrap()` doivent être remplacés par une propagation d'erreur propre avec `?`.

**Concepts à maîtriser :**
- `thiserror` pour définir ses erreurs
- `anyhow` pour les cas où le type exact n'importe pas
- Conversion entre types d'erreurs avec `From`

---

### Phase 3 — Tests unitaires

Écrire des tests pour les fonctions critiques :

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_get_request() {
        // ...
    }

    #[test]
    fn test_router_find_existing_route() {
        // ...
    }

    #[test]
    fn test_router_find_missing_route() {
        // ...
    }
}
```

**Fonctions à tester en priorité :**
- `parse_request` — cas nominaux et cas d'erreur
- `Router::find` — route existante, route manquante, mauvaise méthode
- `Task::new` — avec et sans description

**Commandes utiles :**
```bash
cargo test                    # lancer tous les tests
cargo test test_parse         # lancer un test spécifique
cargo test -- --nocapture     # afficher les println! dans les tests
```

---

### Phase 4 — Async avec Tokio

Remplacer la concurrence par threads avec `tokio` :

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rust
#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_connection(stream).await;
        });
    }
}
```

**Concepts à maîtriser :**
- `async/await`
- `tokio::spawn` vs `std::thread::spawn`
- Pourquoi async est préférable aux threads pour de l'I/O

---

### Phase 5 — Base de données avec sqlx

Remplacer le `Vec<Task>` en mémoire par PostgreSQL :

```toml
[dependencies]
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio"] }
```

**Étapes :**
1. Créer la table `tasks` en SQL
2. Remplacer le `Arc<Mutex<Vec<Task>>>` par un pool de connexions `sqlx::PgPool`
3. Écrire les requêtes SQL dans `task_repository.rs`
4. Gérer les erreurs de BDD dans `AppError`

---

## Endpoints cibles

| Méthode | Route | Description |
|---------|-------|-------------|
| GET | /tasks | Lister toutes les tâches |
| POST | /tasks | Créer une tâche |
| GET | /tasks/:id | Récupérer une tâche |
| PUT | /tasks/:id | Modifier une tâche |
| DELETE | /tasks/:id | Supprimer une tâche |

---

## Dépendances

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "1"
anyhow = "1"
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio"] }
```

---

## Rappels importants

- Pas de `unwrap()` en dehors des tests — utilise `?` et propage les erreurs
- Chaque `unwrap()` qui reste est un bug potentiel en production
- Écris les tests **avant** ou **pendant** le code, pas après
- Commit régulièrement avec des messages clairs
