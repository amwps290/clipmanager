use clipmanager::clipboard::types::ClipboardItem;
use clipmanager::storage::database::Database;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tempfile::NamedTempFile;

fn benchmark_insert_items(c: &mut Criterion) {
    let temp_file = NamedTempFile::new().unwrap();
    let db = Database::new(temp_file.path()).unwrap();

    c.bench_function("insert_100_items", |b| {
        b.iter(|| {
            for i in 0..100 {
                let item = ClipboardItem::new(format!("测试内容 {}", black_box(i)));
                let _ = db.insert_item(&item);
            }
        })
    });
}

fn benchmark_search_items(c: &mut Criterion) {
    let temp_file = NamedTempFile::new().unwrap();
    let db = Database::new(temp_file.path()).unwrap();

    // 预先插入数据
    for i in 0..1000 {
        let item = ClipboardItem::new(format!("搜索测试内容 {}", i));
        let _ = db.insert_item(&item);
    }

    c.bench_function("search_in_1000_items", |b| {
        b.iter(|| {
            let mut filter = clipmanager::clipboard::types::SearchFilter::default();
            filter.query = black_box("测试".to_string());
            let _ = db.get_items(&filter, 50, 0);
        })
    });
}

criterion_group!(benches, benchmark_insert_items, benchmark_search_items);
criterion_main!(benches);
