use async_trait::async_trait;
use casbin::{error::AdapterError, Adapter, Error as CasbinError, Filter, Model, Result};
use sea_orm::ConnectionTrait;

use crate::{
    action::{self, Rule, RuleWithType},
    entity, migration,
};


pub struct SeaOrmAdapter<C> {
    conn: C,
    is_filtered: bool,
}

impl<C: ConnectionTrait> SeaOrmAdapter<C> {
    pub async fn new(conn: C) -> Result<Self> {
        migration::up(&conn)
            .await
            .map(|_| Self {
                conn,
                is_filtered: false,
            })
            .map_err(|err| CasbinError::from(AdapterError(Box::new(err))))
    }

    fn save_policy_line<'a>(ptype: &'a str, rule: &'a [String]) -> Option<RuleWithType<'a>> {
        if ptype.trim().is_empty() || rule.is_empty() {
            return None;
        }

        let mut rule_with_type = RuleWithType {
            ptype,
            v0: "",
            v1: "",
            v2: "",
            v3: "",
            v4: "",
            v5: "",
        };

        #[allow(clippy::get_first)]
        if let Some(v) = rule.get(0) {
            rule_with_type.v0 = v;
        }
        if let Some(v) = rule.get(1) {
            rule_with_type.v1 = v;
        }
        if let Some(v) = rule.get(2) {
            rule_with_type.v2 = v;
        }
        if let Some(v) = rule.get(3) {
            rule_with_type.v3 = v;
        }
        if let Some(v) = rule.get(4) {
            rule_with_type.v4 = v;
        }
        if let Some(v) = rule.get(5) {
            rule_with_type.v5 = v;
        }

        Some(rule_with_type)
    }

    fn load_policy_line(model: &entity::Model) -> Option<Vec<String>> {
        if model.ptype.chars().next().is_some() {
            return Self::normalize_policy(model);
        }

        None
    }

    fn normalize_policy(model: &entity::Model) -> Option<Vec<String>> {
        let mut result = vec![
            &model.v0, &model.v1, &model.v2, &model.v3, &model.v4, &model.v5,
        ];

        while let Some(last) = result.last() {
            if last.is_empty() {
                result.pop();
            } else {
                break;
            }
        }

        if !result.is_empty() {
            return Some(result.iter().map(|&x| x.to_owned()).collect());
        }

        None
    }
}

#[async_trait]
impl<C: ConnectionTrait + Send + Sync> Adapter for SeaOrmAdapter<C> {
    async fn load_policy(&mut self, m: &mut dyn Model) -> Result<()> {
        let rules = action::load_policy(&self.conn).await?;

        for rule in &rules {
            let Some(sec) = rule.ptype.chars().next().map(|x| x.to_string()) else {
                continue;
            };
            let Some(t1) = m.get_mut_model().get_mut(&sec) else {
                continue;
            };
            let Some(t2) = t1.get_mut(&rule.ptype) else {
                continue;
            };
            let Some(line) = Self::load_policy_line(rule) else {
                continue;
            };
            t2.get_mut_policy().insert(line);
        }

        Ok(())
    }

    async fn load_filtered_policy<'a>(&mut self, m: &mut dyn Model, f: Filter<'a>) -> Result<()> {
        let rules = action::load_filtered_policy(&self.conn, &f).await?;
        self.is_filtered = true;

        for rule in &rules {
            let Some(sec) = rule.ptype.chars().next().map(|x| x.to_string()) else {
                continue;
            };
            let Some(t1) = m.get_mut_model().get_mut(&sec) else {
                continue;
            };
            let Some(t2) = t1.get_mut(&rule.ptype) else {
                continue;
            };
            let Some(policy) = Self::normalize_policy(rule) else {
                continue;
            };
            t2.get_mut_policy().insert(policy);
        }

        Ok(())
    }

    async fn save_policy(&mut self, m: &mut dyn Model) -> Result<()> {
        let mut rules = vec![];

        if let Some(ast_map) = m.get_model().get("p") {
            for (ptype, ast) in ast_map {
                let new_rules = ast
                    .get_policy()
                    .into_iter()
                    .filter_map(|x| Self::save_policy_line(ptype, x));

                rules.extend(new_rules);
            }
        }

        if let Some(ast_map) = m.get_model().get("g") {
            for (ptype, ast) in ast_map {
                let new_rules = ast
                    .get_policy()
                    .into_iter()
                    .filter_map(|x| Self::save_policy_line(ptype, x));

                rules.extend(new_rules);
            }
        }
        action::save_policies(&self.conn, &rules).await
    }

    async fn clear_policy(&mut self) -> Result<()> {
        action::clear_policy(&self.conn).await
    }

    fn is_filtered(&self) -> bool {
        self.is_filtered
    }

    async fn add_policy(&mut self, _sec: &str, ptype: &str, rule: Vec<String>) -> Result<bool> {
        let Some(rule_with_type) = Self::save_policy_line(ptype, rule.as_slice()) else {
            return Ok(false);
        };

        action::add_policy(&self.conn, &rule_with_type)
            .await
            .map(|_| true)
    }

    async fn add_policies(
        &mut self,
        _sec: &str,
        ptype: &str,
        rules: Vec<Vec<String>>,
    ) -> Result<bool> {
        let rules = rules
            .iter()
            .filter_map(|x| Self::save_policy_line(ptype, x))
            .collect::<Vec<_>>();

        action::add_policies(&self.conn, &rules).await.map(|_| true)
    }

    async fn remove_policy(&mut self, _sec: &str, ptype: &str, rule: Vec<String>) -> Result<bool> {
        action::remove_policy(&self.conn, ptype, &Rule::from(rule.as_ref())).await
    }

    async fn remove_policies(
        &mut self,
        _sec: &str,
        ptype: &str,
        rules: Vec<Vec<String>>,
    ) -> Result<bool> {
        let rules = rules
            .iter()
            .map(|r| Rule::from(r.as_ref()))
            .collect::<Vec<_>>();
        action::remove_policies(&self.conn, ptype, &rules).await
    }

    async fn remove_filtered_policy(
        &mut self,
        _sec: &str,
        ptype: &str,
        field_index: usize,
        field_values: Vec<String>,
    ) -> Result<bool> {
        if field_index <= 5 && !field_values.is_empty() && field_values.len() >= 6 - field_index {
            Ok(false)
        } else {
            let field_values = if field_values.len() < 6 {
                let mut temp = field_values.clone();
                temp.resize(6, String::new());
                temp
            } else {
                field_values
            };

            let rule: [String; 6] = field_values.try_into().unwrap();
            let mut new_rule: [&str; 6] = Default::default();
            for (i, r) in rule.iter().enumerate() {
                new_rule[i] = r;
            }
            action::remove_filtered_policy(&self.conn, ptype, field_index, &new_rule).await
        }
    }
}