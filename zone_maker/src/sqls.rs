#[allow(dead_code)]
pub const SQL_INSERT_ZONE: &str = r#"
    INSERT INTO `zone_config`(
        `zone_id`,
        `group_id`,
        `gw_port`,
        `http_port`,
        `db_url`,
        `redis_ip`,
        `redis_port`,
        `web_url`,
        `test_blob`,
        `test_timestamp`
    ) 
    VALUES(?,?,?,?,?,?,?,?,?) ON DUPLICATE KEY UPDATE 
        `gw_port`=?,
        `http_port`=?,
        `db_url`=?,
        `redis_ip`=?,
        `redis_port`=?,
        `web_url`=?,
        `test_blob`=?,
        `test_timestamp`=?
"#;

#[allow(dead_code)]
pub const SQL_QUERY_ZONE: &str = r#"SELECT * FROM `zone_config` WHERE zone_id=?"#;
