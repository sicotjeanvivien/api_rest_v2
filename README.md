# API REST V2 – Rust

API REST moderne développée en **Rust**, avec une architecture inspirée des principes **Clean Architecture / Hexagonal Architecture**.
Le projet sert de base pour construire des APIs robustes, typées et performantes avec une séparation claire des responsabilités entre le domaine, l'application et l'infrastructure.

L'objectif est de séparer clairement :
- la **logique métier**
- les **cas d'usage applicatifs**
- l'**infrastructure technique**
- les **interfaces externes (HTTP)**
Chaque couche dépend uniquement des couches **plus internes**.  

---
# Objectifs du projet
- Construire une API REST idiomatique en Rust
- Appliquer des principes d’architecture propres (Domain Driven Design, Ports & Adapters)
- Mettre en place une gestion d’erreurs centralisée
- Structurer un projet backend Rust maintenable et testable
- Servir de base réutilisable pour d'autres projets API

---
# Stack technique
- **Rust**
- **Tokio** – Runtime async
- **SQLx** – Accès base de données
- **PostgreSQL**
- **Serde** – Sérialisation JSON
- **Custom Router / HTTP layer**
- **Async Trait**

---
# Architecture
Le projet suit une organisation en couches :

```
└── 📁src
    └── 📁application
        └── 📁services
            ├── mod.rs
            ├── task_service.rs
        ├── mod.rs
    └── 📁domain
        └── 📁error
            ├── mod.rs
            ├── repository_error.rs
        └── 📁task
            ├── mod.rs
            ├── model.rs
            ├── repository.rs
        ├── mod.rs
    └── 📁infra
        └── 📁stores
            ├── in_memory_task_store.rs
            ├── mod.rs
            ├── postgres_task_store.rs
        ├── mod.rs
    └── 📁interface
        └── 📁http
            └── 📁dto
                └── 📁request
                    ├── create_task_request.rs
                    ├── mod.rs
                └── 📁response
                    ├── mod.rs
                    ├── task_response.rs
                ├── mod.rs
            └── 📁error
                ├── mod.rs
            └── 📁handlers
                ├── error_handler.rs
                ├── mod.rs
                ├── task_handler.rs
            └── 📁request
                ├── mod.rs
            └── 📁response
                ├── http_response.rs
                ├── into_http_response.rs
                ├── mod.rs
                ├── status_code.rs
            └── 📁router
                ├── macros.rs
                ├── mod.rs
                ├── route.rs
            ├── mod.rs
            ├── parser.rs
        ├── mod.rs
    └── 📁migrations
        ├── 20240101000000_create_tasks.sql
    └── main.rs
```

# Description des couches
## Domain

Contient la **logique métier pure**. Cette couche est totalement indépendante du reste du système.

Elle contient :
- les **modèles métier**
- les **interfaces (traits) des repositories**
- les **erreurs métier**
  
Exemple :
```
domain/task/model.rs
domain/task/repository.rs
````

Le `repository.rs` définit uniquement le **contrat** :

```rust

pub trait TaskRepository {
    fn get(&self, id: i32) -> Result<Task, RepositoryError>;
}
````
Aucune implémentation concrète n'est présente dans cette couche.

---
## Application

Contient les **services applicatifs** qui orchestrent les cas d’usage.
Les services :
* utilisent les **repositories du domaine**
* implémentent la **logique applicative**

Exemple :
```
application/services/task_service.rs
```
Le service agit comme un **point d'entrée métier** pour les handlers HTTP.

---
## Infrastructure (infra)

Implémente les interfaces définies dans le domaine.
Exemple :
```
infra/stores/postgres_task_store.rs
infra/stores/in_memory_task_store.rs
```

Cela permet de changer l’implémentation sans modifier le domaine :
* PostgreSQL
* InMemory (tests)
* Redis
* autre backend

---
## Interface HTTP

Couche responsable de l'exposition de l'API. Elle contient :
### DTO

Objets utilisés pour l'entrée / sortie HTTP.
```
dto/request
dto/response
```
Ils permettent d’éviter d’exposer directement les modèles métier.
### Handlers

Les handlers reçoivent les requêtes HTTP et appellent les services applicatifs.
Exemple :
```
handlers/task_handler.rs
```

Flux typique :
```
HTTP Request
   ↓
Handler
   ↓
Service
   ↓
Repository
   ↓
Database
```
### Router
Le router associe les routes HTTP aux handlers.

```
router/route.rs
```
### Response

Gestion des réponses HTTP :

```
http_response.rs
status_code.rs
into_http_response.rs
```

Permet de construire des réponses HTTP typées.

---
# Flux d'une requête

```
Client HTTP
     │
     ▼
Router
     │
     ▼
Handler
     │
     ▼
Service (Application)
     │
     ▼
Repository Trait (Domain)
     │
     ▼
Repository Implementation (Infra)
     │
     ▼
Database
```

---
# Migration base de données

Les migrations SQL sont stockées dans : 

```
src/migrations
```

Exemple :
```
20240101000000_create_tasks.sql
```

---
# Avantages de cette architecture

* séparation claire des responsabilités
* testabilité élevée
* infrastructure interchangeable
* domaine indépendant
* code maintenable
* 
Cette structure permet de faire évoluer le projet sans coupler la logique métier à l'infrastructure.

---
---
# Modèle Task

Exemple d'entité exposée par l'API.
```rust
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
}
````

---
# Routes API

| Method | Route         | Description                 |
| ------ | ------------- | --------------------------- |
| GET    | `/tasks`      | Récupérer toutes les tâches |
| GET    | `/tasks/{id}` | Récupérer une tâche         |
| POST   | `/tasks`      | Créer une tâche             |
| PATCH  | `/tasks/{id}` | Mettre à jour une tâche     |
| DELETE | `/tasks/{id}` | Supprimer une tâche         |

---
# Installation
### Prérequis
* Rust
* PostgreSQL
* Cargo
---
# Configuration

Créer un fichier `.env`
```
DATABASE_URL=postgres://user:password@localhost:5432/tasks
```

---
# Lancer le projet

```bash
cargo run
```

---
# Lancer les tests

```bash
cargo test
```

---
# Migration base de données

Si SQLx est utilisé avec migrations :
```bash
sqlx migrate run
```

---
# Philosophie du projet

Ce projet met l'accent sur :
* **séparation claire des responsabilités**
* **testabilité**
* **code explicite**
* **typage fort**
* **erreurs gérées proprement**
Rust permet de construire des APIs **fiables et performantes**, tout en conservant une architecture propre comparable aux standards du backend moderne.

---
# Licence

MIT License

Copyright (c) 2026 Jean-Vivien Sicot

Permission is hereby granted, free of charge, to any person obtaining a copy