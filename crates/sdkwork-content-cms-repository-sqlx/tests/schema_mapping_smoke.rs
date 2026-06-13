use sdkwork_content_cms_repository_sqlx::db::schema::CMS_V1_TABLES;

#[test]
fn cms_v1_schema_declares_expected_table_count() {
    assert_eq!(CMS_V1_TABLES.len(), 22);
    assert!(CMS_V1_TABLES.contains(&"cms_entry"));
    assert!(CMS_V1_TABLES.contains(&"cms_feed_snapshot"));
}
