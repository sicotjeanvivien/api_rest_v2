// infra/benches/task_store.rs
use criterion::{Criterion, criterion_group, criterion_main};
use domain::{NewTask, TaskRepository};
use infra::PostgresTaskStore;
use tokio::runtime::Runtime;

fn bench_get_all(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    // Setup — créer le store une seule fois
    let store = rt.block_on(async { PostgresTaskStore::new().await });

    c.bench_function("get_all_tasks", |b| {
        b.to_async(&rt)
            .iter(|| async { store.get_all().await.unwrap() })
    });
}

fn bench_get_by_id(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let store = rt.block_on(async { PostgresTaskStore::new().await });

    c.bench_function("get_task_by_id", |b| {
        b.to_async(&rt)
            .iter(|| async { store.get(4).await.unwrap() })
    });
}

fn bench_create(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let store = rt.block_on(async { PostgresTaskStore::new().await });

    c.bench_function("create", |b| {
        b.to_async(&rt).iter(|| async {
            store
                .create(NewTask {
                    title: String::from("test task"),
                    description: Some(String::from("description")),
                })
                .await
                .unwrap()
        });
    });
}

fn bench_deleted(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let store = rt.block_on(async { PostgresTaskStore::new().await });

    c.bench_function("create", |b| {
        b.to_async(&rt).iter(|| async {
            store
                .delete(20)
                .await
                .unwrap()
        });
    });
}

criterion_group!(benches, bench_create, bench_get_all, bench_get_by_id, bench_deleted);
criterion_main!(benches);
