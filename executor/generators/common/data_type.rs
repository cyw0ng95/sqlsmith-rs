use sqlsmith_rs_common::rand_by_seed::LcgRng;

pub fn generate_value_by_type(ty: &str, rng: &mut LcgRng) -> String {
    match ty.to_uppercase().as_str() {
        "INTEGER" => (rng.rand().abs() % 1000).to_string(),
        "REAL" => format!("{}", (rng.rand().abs() as f64) / 100.0),
        "TEXT" => format!("'val{}'", rng.rand().abs() % 1000),
        "BLOB" => {
            // 生成 1 到 16 字节的随机 BLOB
            let len = (rng.rand().unsigned_abs() % 16) + 1;
            let mut blob = Vec::with_capacity(len as usize);
            for _ in 0..len {
                blob.push((rng.rand().unsigned_abs() % 256) as u8);
            }
            let hex_str = blob.iter().map(|b| format!("{:02x}", b)).collect::<String>();
            format!("X'{}'", hex_str)
        }
        _ => "NULL".to_string(),
    }
}