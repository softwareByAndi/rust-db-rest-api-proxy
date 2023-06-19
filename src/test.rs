use tokio_postgres::types::{FromSql, Type, IsNull, ToSql};
use std::error::Error;

enum SqlValue {
    Text(String),
    Integer(i32),
    // Add more variants for other SQL types as needed
}

impl<'a> FromSql<'a> for SqlValue {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        match ty {
            &Type::TEXT => {
                let value = std::str::from_utf8(raw)?.to_owned();
                Ok(SqlValue::Text(value))
            }
            &Type::INT4 => {
                let value = i32::from_sql(ty, raw)?;
                Ok(SqlValue::Integer(value))
            }
            // Add more match cases for other SQL types as needed
            _ => Err(format!("Unsupported SQL type: {:?}", ty).into()),
        }
    }

    fn accepts(ty: &Type) -> bool {
        // Accept any type for this example, but you can restrict specific types if needed
        true
    }
}

impl ToSql for SqlValue {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut Vec<u8>,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        match self {
            SqlValue::Text(value) => {
                let value = value.as_str();
                value.to_sql(ty, out)
            }
            SqlValue::Integer(value) => {
                value.to_sql(ty, out)
            }
            // Add more match cases for other SQL types as needed
        }
    }

    fn accepts(ty: &Type) -> bool {
        // Accept any type for this example, but you can restrict specific types if needed
        true
    }

    fn type_info(&self) -> Option<Type> {
        match self {
            SqlValue::Text(_) => Some(Type::TEXT),
            SqlValue::Integer(_) => Some(Type::INT4),
            // Add more match cases for other SQL types as needed
        }
    }
}

// Example usage
fn main() {
    let values: Vec<SqlValue> = vec![
        SqlValue::Text("Hello".to_owned()),
        SqlValue::Integer(42),
    ];

    for value in values {
        match value {
            SqlValue::Text(text) => {
                println!("Text: {}", text);
            }
            SqlValue::Integer(integer) => {
                println!("Integer: {}", integer);
            }
            // Add more match cases for other SQL types as needed
        }
    }
}
