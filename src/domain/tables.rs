use rbatis::rbdc::DateTime;
use rbatis::table_sync::{
    ColumMapper, MssqlTableMapper, MysqlTableMapper, PGTableMapper, SqliteTableMapper,
};
use rbatis::RBatis;
use super::enums::LoginCheck;


#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CasbinRule {
    pub id: Option<String>,
    pub ptype: Option<String>,
    pub v0: Option<String>,
    pub v1: Option<String>,
    pub v2: Option<String>,
    pub v3: Option<String>,
    pub v4: Option<String>,
    pub v5: Option<String>,
}


#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysUser {
    pub id: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub login_check: Option<LoginCheck>,
    pub state: Option<i32>,
    pub create_date: Option<DateTime>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysDict {
    pub id: Option<String>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub state: Option<i32>,
    pub create_date: Option<DateTime>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysTrash {
    pub id: Option<String>,
    pub table_name: Option<String>,
    pub data: Option<String>,
    pub create_date: Option<DateTime>,
}


pub async fn sync_tables(rb: &RBatis) {
    let mapper = {
        match rb.driver_type().unwrap() {
            "sqlite" => &SqliteTableMapper {} as &dyn ColumMapper,
            "mssql" => &MssqlTableMapper {} as &dyn ColumMapper,
            "mysql" => &MysqlTableMapper {} as &dyn ColumMapper,
            "postgres" => &PGTableMapper {} as &dyn ColumMapper,
            _ => {
                panic!("not find driver mapper")
            }
        }
    };
    let conn = rb.acquire().await.expect("connection database fail");
    let table = CasbinRule {
        id: Some("".to_string()),
        ptype: Some("".to_string()),
        v0: Some("".to_string()),
        v1: Some("".to_string()),
        v2: Some("".to_string()),
        v3: Some("".to_string()),
        v4: Some("".to_string()),
        v5: Some("".to_string()),
    };
    let _ = RBatis::sync(&conn, mapper, &table, "casbin_rule").await;
    let table = SysUser {
        id: Some("".to_string()),
        account: Some("".to_string()),
        password: Some("".to_string()),
        name: Some("".to_string()),
        login_check: Some(LoginCheck::NoCheck),
        state: Some(0),
        create_date: Some(DateTime::now()),
    };
    let _ = RBatis::sync(&conn, mapper, &table, "sys_user").await;
    let table = SysDict {
        id: Some("".to_string()),
        name: Some("".to_string()),
        code: Some("".to_string()),
        state: Some(0),
        create_date: Some(DateTime::now()),
    };
    let _ = RBatis::sync(&conn, mapper, &table, "sys_dict").await;
    let table = SysTrash {
        id: Some("".to_string()),
        table_name: Some("".to_string()),
        data: Some("".to_string()),
        create_date: Some(DateTime::now()),
    };
    let _ = RBatis::sync(&conn, mapper, &table, "sys_trash").await;
}