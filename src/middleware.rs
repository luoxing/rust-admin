use casbin::{Adapter, Filter, Model, Result};
use casbin::{error::AdapterError, error::Error as CasbinError};
use rbatis::executor::Executor;
use rbatis::RBatis;
use rbs::to_value;

use crate::domain::tables::CasbinRule;

#[derive(Debug)]
pub struct CasbinAdapter {
    rb: RBatis,
    is_filtered: bool,
}

impl CasbinAdapter {
    pub async fn new(rb: RBatis, is_filtered: bool) -> Result<Self> {
        let this = Self {
            rb: rb.clone(),
            is_filtered: false,
        };
        Ok(this)
    }

    pub async fn clear_policy(&self) -> Result<()> {
        let sql_statment = format!("delete from casbin_rule");

        self.rb.query_decode(sql_statment.as_str(), vec![])
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(err))))?;
    Result::Ok(())
    }
    
    pub async fn save_policy(&self,rules: Vec<CasbinRule>) -> Result<()> {
        let mut tx = self.rb.acquire_begin().await.map_err(|err| CasbinError::from(AdapterError(Box::new(err))))?;
    
        for rule in rules {
            CasbinRule::insert(&mut tx, &rule)
                .await
                .map_err(|err| CasbinError::from(AdapterError(Box::new(err))))?;
        }
        tx.commit().await.map_err(|err| CasbinError::from(AdapterError(Box::new(err))))?;
        Ok(())
    }
    
    #[py_sql(
        "`delete from casbin_rule`
            where:
                for k,rule in rules:
                    ` ptype = #{ptype}`
                    for key,item in rule:
                        ` and v${key} = #{item}` "
    )]
    async fn remove_policies_sql(rb: &mut dyn Executor, ptype: &str, rules: &Vec<Vec<String>>) -> rbatis::Result<()> {}
    
    pub async fn remove_policy(self, pt: &str, rule: Vec<String>) -> Result<bool> {
        self.remove_policies(pt, vec![rule]).await
    }
    
    pub async fn remove_policies(self, pt: &str, rules: Vec<Vec<String>>) -> Result<bool> {
        let mut normal_rules = vec![];
    
        for rule in rules {
            normal_rules.push(Self::normalize_casbin_rule(rule, 0))
        }
    
        Self::remove_policies_sql(&mut self.rb.clone(), pt, &normal_rules)
            .await
            .map_err(|err| CasbinError::from(AdapterError(Box::new(err))))
            .map(|_| true)
    }
    
    pub async fn remove_filtered_policy(rb: &RBatis, pt: &str, field_index: usize, field_values: Vec<String>) -> Result<bool> {
        let field_values = Self::normalize_casbin_rule(field_values, field_index);
    
        let (sql, parameters) = if field_index == 5 {
            let sql = "DELETE FROM casbin_rule WHERE ptype = ? AND (v5 is NULL OR v5 = COALESCE(?,v5))";
            let p = vec![to_value!(pt.to_string()), to_value!(field_values[0].to_string())];
            (sql, p)
        } else if field_index == 4 {
            let sql = "DELETE FROM casbin_rule WHERE
            ptype = ? AND
            (v4 is NULL OR v4 = COALESCE(?,v4)) AND
            (v5 is NULL OR v5 = COALESCE(?,v5))";
    
            let p = vec![
                to_value!(pt.to_string()),
                to_value!(field_values[0].to_string()),
                to_value!(field_values[1].to_string()),
            ];
            (sql, p)
        } else if field_index == 3 {
            let sql = "DELETE FROM casbin_rule WHERE
            ptype = ? AND
            (v3 is NULL OR v3 = COALESCE(?,v3)) AND
            (v4 is NULL OR v4 = COALESCE(?,v4)) AND
            (v5 is NULL OR v5 = COALESCE(?,v5))";
            let p = vec![
                to_value!(pt.to_string()),
                to_value!(field_values[0].to_string()),
                to_value!(field_values[1].to_string()),
                to_value!(field_values[2].to_string()),
            ];
            (sql, p)
        } else if field_index == 2 {
            let sql = "DELETE FROM casbin_rule WHERE
            ptype = ? AND
            (v2 is NULL OR v2 = COALESCE(?,v2)) AND
            (v3 is NULL OR v3 = COALESCE(?,v3)) AND
            (v4 is NULL OR v4 = COALESCE(?,v4)) AND
            (v5 is NULL OR v5 = COALESCE(?,v5))";
            let p = vec![
                to_value!(pt.to_string()),
                to_value!(field_values[0].to_string()),
                to_value!(field_values[1].to_string()),
                to_value!(field_values[2].to_string()),
                to_value!(field_values[3].to_string()),
            ];
            (sql, p)
        } else if field_index == 1 {
            let sql = "DELETE FROM casbin_rule WHERE
            ptype = ? AND
            (v1 is NULL OR v1 = COALESCE(?,v1)) AND
            (v2 is NULL OR v2 = COALESCE(?,v2)) AND
            (v3 is NULL OR v3 = COALESCE(?,v3)) AND
            (v4 is NULL OR v4 = COALESCE(?,v4)) AND
            (v5 is NULL OR v5 = COALESCE(?,v5))";
            let p = vec![
                to_value!(pt.to_string()),
                to_value!(field_values[0].to_string()),
                to_value!(field_values[1].to_string()),
                to_value!(field_values[2].to_string()),
                to_value!(field_values[3].to_string()),
                to_value!(field_values[4].to_string()),
            ];
            (sql, p)
        } else {
            let sql = "DELETE FROM casbin_rule WHERE
            ptype = ? AND
            (v0 is NULL OR v0 = COALESCE(?,v0)) AND
            (v1 is NULL OR v1 = COALESCE(?,v1)) AND
            (v2 is NULL OR v2 = COALESCE(?,v2)) AND
            (v3 is NULL OR v3 = COALESCE(?,v3)) AND
            (v4 is NULL OR v4 = COALESCE(?,v4)) AND
            (v5 is NULL OR v5 = COALESCE(?,v5))";
            let p = vec![
                to_value!(pt.to_string()),
                to_value!(field_values[0].to_string()),
                to_value!(field_values[1].to_string()),
                to_value!(field_values[2].to_string()),
                to_value!(field_values[3].to_string()),
                to_value!(field_values[4].to_string()),
                to_value!(field_values[5].to_string()),
            ];
            (sql, p)
        };
        rb.query_decode::<bool>(sql, parameters)
            .await
            .map_err(|err| CasbinError::from(AdapterError(Box::new(err))))
    }
    
    pub(crate) async fn load_policy(rb: &mut RBatis) -> Result<Vec<CasbinRule>> {
        let vec_rules = CasbinRule::select_all(rb).await.map_err(|err| CasbinError::from(AdapterError(Box::new(err))))?;
        Result::Ok(vec_rules)
    }
    
    pub(crate) async fn add_policy(rb: &mut RBatis, new_rule: CasbinRule) -> Result<bool> {
        CasbinRule::insert(rb, &new_rule)
            .await
            .map_err(|err| CasbinError::from(AdapterError(Box::new(err))))?;
        Result::Ok(true)
    }
    
    pub(crate) async fn add_policies(rb: &RBatis, rules: Vec<CasbinRule>) -> Result<bool> {
        let mut tx = rb.acquire_begin().await.map_err(|err| CasbinError::from(AdapterError(Box::new(err))))?;
    
        for rule in rules {
            CasbinRule::insert(&mut tx, &rule)
                .await
                .map_err(|err| CasbinError::from(AdapterError(Box::new(err))))?;
        }
        tx.commit().await.map_err(|err| CasbinError::from(AdapterError(Box::new(err))))?;
        Ok(true)
    }
    
    fn normalize_casbin_rule(mut rule: Vec<String>, field_index: usize) -> Vec<String> {
        rule.resize(6 - field_index, String::from(""));
        rule
    }
}

impl Adapter for CasbinAdapter{
    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn load_policy<'life0,'life1,'async_trait>(&'life0 mut self,m: &'life1 mut dyn Model) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait,Self:'async_trait {
    todo!()
}

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn load_filtered_policy<'a,'life0,'life1,'async_trait>(&'life0 mut self,m: &'life1 mut dyn Model,f:Filter<'a> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'a:'async_trait+ ,'life0:'async_trait,'life1:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn save_policy<'life0,'life1,'async_trait>(&'life0 mut self,m: &'life1 mut dyn Model) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn clear_policy<'life0,'async_trait>(&'life0 mut self) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn is_filtered(&self) -> bool {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn add_policy<'life0,'life1,'life2,'async_trait>(&'life0 mut self,sec: &'life1 str,ptype: &'life2 str,rule:Vec<String> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<bool> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait,'life2:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn add_policies<'life0,'life1,'life2,'async_trait>(&'life0 mut self,sec: &'life1 str,ptype: &'life2 str,rules:Vec<Vec<String> > ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<bool> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait,'life2:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn remove_policy<'life0,'life1,'life2,'async_trait>(&'life0 mut self,sec: &'life1 str,ptype: &'life2 str,rule:Vec<String> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<bool> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait,'life2:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn remove_policies<'life0,'life1,'life2,'async_trait>(&'life0 mut self,sec: &'life1 str,ptype: &'life2 str,rules:Vec<Vec<String> > ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<bool> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait,'life2:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn remove_filtered_policy<'life0,'life1,'life2,'async_trait>(&'life0 mut self,sec: &'life1 str,ptype: &'life2 str,field_index:usize,field_values:Vec<String> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<bool> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait,'life2:'async_trait,Self:'async_trait {
        todo!()
    }
}