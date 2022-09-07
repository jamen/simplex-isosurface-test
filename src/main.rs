use clatter::Simplex3d;
use isosurface::{
    distance::Signed, extractor::Extractor, math::Vec3, sampler::Sample, MarchingCubes,
};

struct SimplexSource {
    sampler: Simplex3d,
    scale: f32,
}

impl SimplexSource {
    fn new(scale: f32) -> Self {
        Self {
            sampler: Simplex3d::new(),
            scale,
        }
    }
}

impl Sample<Signed> for SimplexSource {
    fn sample(&self, v: Vec3) -> Signed {
        let s = self.scale;
        Signed(
            self.sampler
                .sample([[v.x * s].into(), [v.y * s].into(), [v.z * s].into()])
                .value[0],
        )
    }
}

#[derive(Default)]
struct Mesh {
    vertices: Vec<Vec3>,
    indices: Vec<usize>,
}

impl Extractor for Mesh {
    fn extract_vertex(&mut self, vertex: Vec3) {
        self.vertices.push(vertex);
    }
    fn extract_index(&mut self, index: usize) {
        self.indices.push(index);
    }
}

fn main() {
    let simplex_source = SimplexSource::new(3.0);
    let mut marching_cubes = MarchingCubes::<Signed>::new(64);
    let mut mesh = Mesh::default();

    marching_cubes.extract(&simplex_source, &mut mesh);

    for vertex in mesh.vertices.iter() {
        println!("v {} {} {}", vertex.x, vertex.y, vertex.z);
    }
    for i in (0..mesh.indices.len()).step_by(3) {
        let i1 = mesh.indices[i] + 1;
        let i2 = mesh.indices[i + 1] + 1;
        let i3 = mesh.indices[i + 2] + 1;
        println!("f {} {} {}", i1, i2, i3);
    }
}
