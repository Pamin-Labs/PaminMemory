//! Boots a real embedded PostgreSQL and applies the migrations.
//!
//! Ignored by default: the first run downloads and installs a PostgreSQL
//! distribution, which is too slow and too network-dependent for the ordinary
//! test loop. Run it with `cargo test -p pamin-store -- --ignored`.

use pamin_store::{Database, Workspace};

#[tokio::test]
#[ignore = "downloads and starts a real postgres cluster"]
async fn bootstrap_creates_a_usable_migrated_database() {
    let dir = tempfile::tempdir().expect("temp workspace");
    let workspace = Workspace::at(dir.path());

    let database = Database::open(&workspace).await.expect("open workspace");

    // Every table the first migration creates should be present and queryable.
    for table in [
        "projects",
        "sources",
        "source_versions",
        "source_spans",
        "topics",
        "topic_states",
        "index_jobs",
    ] {
        let row = database
            .client()
            .query_one(&format!("SELECT count(*) FROM {table}"), &[])
            .await
            .unwrap_or_else(|error| panic!("querying {table}: {error}"));
        let count: i64 = row.get(0);
        assert_eq!(count, 0, "{table} should start empty");
    }

    // Re-opening must reuse the running server rather than starting a second
    // one, and must not fail by re-applying migrations.
    let reopened = Database::open(&workspace).await.expect("reopen workspace");
    drop(reopened);
    drop(database);

    pamin_store::database::stop(&workspace)
        .await
        .expect("stop server");
}
