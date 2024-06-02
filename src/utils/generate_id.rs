use rand::Rng;

pub fn generate_id() -> String {
    let mut rng = rand::thread_rng();
    let id: u32 = rng.gen_range(100000..1000000);
    id.to_string()
}
