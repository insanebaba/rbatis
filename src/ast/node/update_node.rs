use serde_json::{json, Value};

use rbatis_core::convert::StmtConvert;

use crate::ast::ast::RbatisAST;
use crate::ast::node::node::{create_deep, do_child_nodes, print_child, SqlNodePrint};
use crate::ast::node::node_type::NodeType;
use crate::engine::runtime::RbatisEngine;

#[derive(Clone, Debug)]
pub struct UpdateNode {
    pub id: String,
    pub childs: Vec<NodeType>,
}


impl RbatisAST for UpdateNode {
    fn eval(&self, convert: &impl StmtConvert, env: &mut Value, engine: &RbatisEngine, arg_array: &mut Vec<Value>) -> Result<String, rbatis_core::Error> {
        return do_child_nodes(convert, &self.childs, env, engine, arg_array);
    }
}

impl SqlNodePrint for UpdateNode {
    fn print(&self, deep: i32) -> String {
        let mut result = create_deep(deep) + "<update ";
        result = result + "id=\"" + self.id.as_str() + "\"";
        result = result + ">";
        result = result + print_child(self.childs.as_ref(), deep + 1).as_str();
        result = result + create_deep(deep).as_str() + "</update>";
        return result;
    }
}