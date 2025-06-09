use sqlsmith_rs_common::rand_by_seed::LcgRng;

pub fn generate_value_by_type(ty: &str, rng: &mut LcgRng) -> String {
    match ty.to_uppercase().as_str() {
        "INTEGER" => (rng.rand().abs() % 1000).to_string(),
        "REAL" => format!("{}", (rng.rand().abs() as f64) / 100.0),
        "TEXT" => format!("'val{}'", rng.rand().abs() % 1000),
        "BLOB" => "'blob'".to_string(),
        _ => "NULL".to_string(),
    }
}