struct Euclidean;
impl space::Metric<&[f32]> for Euclidean {
    type Unit = u32;
    fn distance(&self, a: &&[f32], b: &&[f32]) -> u32 {
        a.iter()
            .zip(b.iter())
            .map(|(&a, &b)| (a - b) * (a - b))
            .sum::<f32>()
            .sqrt()
            .to_bits()
    }
}
type MyHnsw<'a> = hnsw::Hnsw<Euclidean, &'a [f32], rand_chacha::ChaCha20Rng, 10, 10>;
fn main() {
    let mut hnsw: MyHnsw = MyHnsw::new(Euclidean);
    let mut searcher: hnsw::Searcher<u32> = Default::default();
    let q: &[f32] = &[0.0, 1.1, 2.2];
    hnsw.insert(q, &mut searcher);
}
