# API REST V2 – Rust

API REST modulaire développée en **Rust**, basée sur une architecture **Clean / Hexagonale**.  
Le projet est conçu pour être **testable, découplé et maintenable**, tout en exploitant les performances et la sûreté de Rust.

---

## 🚀 Quick Start

### Prérequis

- Rust (>= 1.75)
- PostgreSQL

### Installation

```bash
git clone https://github.com/ton-repo/api_rest_v2.git
cd api_rest_v2
cp .env.example .env
````

### Configuration

```env
DATABASE_URL=postgres://user:password@localhost:5432/tasks
```

### Lancer l'application

```bash
cargo run
```

### Tester rapidement

```bash
curl http://localhost:3000/tasks
```

---

## 🎯 Objectif

Ce projet vise à :

* construire une API REST idiomatique en Rust
* appliquer les principes de **Clean Architecture / DDD**
* garantir une forte **testabilité**
* proposer une base réutilisable pour des APIs backend modernes

---

## ✨ Features

* 🧱 Architecture hexagonale (Domain / Application / Infra / Interface)
* ⚡ Runtime asynchrone avec Tokio
* 🔌 Injection de dépendances via traits
* 🗄️ Support PostgreSQL + InMemory
* 🔒 Gestion d’erreurs centralisée
* 🧪 Tests avancés (unit + property-based + mocks)
* 🔐 Authentification JWT

---

## 🧱 Architecture

Le projet suit une séparation stricte des responsabilités :

* **Domain** : logique métier pure (sans dépendances externes)
* **Application** : orchestration des cas d’usage
* **Infrastructure** : implémentations techniques (DB, stockage)
* **Interface** : exposition HTTP (handlers, routing)

### Flux d'une requête

```
HTTP → Handler → Service → Domain → Repository → Database
```

---

## 📁 Project Structure

Organisation en workspace multi-crates :

```
api_rest_v2/
├── domain          # logique métier
├── application     # cas d’usage
├── infra           # implémentations (Postgres, InMemory)
├── interface       # HTTP (handlers, router, DTO)
├── bootstrap       # initialisation (server, DI, router)
├── derive_macros   # macros custom
```

---

## ⚙️ Stack technique

* **Rust**
* **Tokio**
* **SQLx**
* **PostgreSQL**
* **Serde**

---

## 🔁 Exemple d’utilisation

### Créer une tâche

```bash
curl -X POST http://localhost:3000/tasks \
  -H "Content-Type: application/json" \
  -d '{"title": "Test task"}'
```

### Réponse

```json
{
  "id": 1,
  "title": "Test task",
  "done": false
}
```

---

## 🧪 Testing

Lancer les tests :

```bash
cargo test
```

Le projet inclut :

* unit tests
* property-based testing (`proptest`)
* mocking (`mockall`)

---

## 🗄️ Migrations

```bash
sqlx migrate run
```

Les fichiers sont situés dans :

```
migrations/
```

---

## 🧠 Design Decisions

### Pourquoi une Clean Architecture ?

Permet de découpler complètement la logique métier de l’infrastructure, facilitant :

* les tests
* la maintenance
* l’évolution du système

### Pourquoi des traits pour les repositories ?

Les traits permettent :

* l’injection de dépendances
* le remplacement d’implémentations (Postgres, InMemory…)
* une meilleure testabilité

### Pourquoi un router HTTP custom ?

Permet de :

* comprendre finement le fonctionnement HTTP
* garder un contrôle total sur la stack
* éviter une dépendance forte à un framework

---

## 📌 Roadmap

* [ ] Observabilité (tracing, metrics)
* [ ] Dockerisation
* [ ] Rate limiting
* [ ] CI/CD
* [ ] Documentation OpenAPI

---

## 🤝 Contributing

Les contributions sont les bienvenues :

1. Fork du projet
2. Création d’une branche
3. Pull Request

---

## 📄 Licence

MIT License

Copyright (c) 2026 Jean-Vivien Sicot
