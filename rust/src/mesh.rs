pub mod geometry;

use cgmath::InnerSpace;
use three_d::{Context, ElementBuffer, Vec3, VertexBuffer};

#[derive(Clone)]
pub struct CpuMesh {
    pub positions: Vec<Vec3>,
    pub indices: Vec<u32>,
}

impl CpuMesh {
    pub fn new(positions: Vec<Vec3>, indices: Option<Vec<u32>>) -> Self {
        let indices = match indices {
            Some(indices) => indices,
            None => Vec::from_iter(0..positions.len() as u32),
        };
        Self { positions, indices }
    }

    pub fn face(&mut self, point: Vec3, away: bool) {
        for i in 0..self.indices.len() / 3 {
            let i = i * 3;
            let ia = self.indices[i + 0] as usize;
            let ib = self.indices[i + 1] as usize;
            let ic = self.indices[i + 2] as usize;
            let a = self.positions[ia];
            let b = self.positions[ib];
            let c = self.positions[ic];
            let normal = (b - a).cross(c - a);
            let dot = normal.dot(point - a);
            if away != (dot < 0.0) {
                self.indices.swap(i + 1, i + 2);
            }
        }
    }

    pub fn _face_toward(&mut self, point: Vec3) {
        self.face(point, false)
    }

    pub fn face_away(&mut self, point: Vec3) {
        self.face(point, true)
    }

    pub fn subdivide(&self) -> Self {
        let mut new_positions = self.positions.clone();
        let mut new_indices = Vec::new();
        for i in 0..self.indices.len() / 3 {
            let i = i * 3;
            let ia = self.indices[i + 0];
            let ib = self.indices[i + 1];
            let ic = self.indices[i + 2];
            let a = self.positions[ia as usize];
            let b = self.positions[ib as usize];
            let c = self.positions[ic as usize];
            let ab = (a + b) / 2.0;
            let bc = (b + c) / 2.0;
            let ca = (c + a) / 2.0;
            let iab = new_positions.len() as u32;
            let ibc = iab + 1;
            let ica = ibc + 1;
            new_positions.extend(&[ab, bc, ca]);
            new_indices.extend(&[ia, iab, ica, ib, ibc, iab, ic, ica, ibc, iab, ibc, ica]);
        }
        Self {
            positions: new_positions,
            indices: new_indices,
        }
    }

    pub fn subdivide_n(&self, n: u32) -> Self {
        let mut mesh = self.clone();
        for _ in 0..n {
            mesh = mesh.subdivide();
        }
        mesh
    }

    pub fn split_triangles(&self) -> Self {
        Self {
            positions: self
                .indices
                .iter()
                .map(|&i| self.positions[i as usize])
                .collect(),
            indices: Vec::from_iter(0..self.indices.len() as u32),
        }
    }
}

pub struct GpuMesh {
    pub positions: VertexBuffer,
    pub indices: ElementBuffer,
}

impl GpuMesh {
    fn new(context: &Context, cpu_mesh: &CpuMesh) -> Self {
        let positions = VertexBuffer::new_with_data(context, &cpu_mesh.positions);
        let indices = ElementBuffer::new_with_data(context, &cpu_mesh.indices);
        GpuMesh { positions, indices }
    }

    fn fill(&mut self, mesh: &CpuMesh) {
        self.positions.fill(&mesh.positions);
        self.indices.fill(&mesh.indices);
    }
}

pub struct Mesh {
    pub cpu_mesh: CpuMesh,
    pub gpu_mesh: GpuMesh,
}

impl Mesh {
    pub fn new(context: &Context, cpu_mesh: CpuMesh) -> Self {
        let gpu_mesh = GpuMesh::new(context, &cpu_mesh);
        Self { cpu_mesh, gpu_mesh }
    }

    pub fn from_mesh_embedded<F>(context: &Context, mesh: &Mesh, f: F) -> Self
    where
        F: Fn(Vec3) -> Vec3,
    {
        let mut mesh = Self::new(context, mesh.cpu_mesh.clone());
        mesh.embed(f);
        mesh
    }

    pub fn from_positions(context: &Context, positions: Vec<Vec3>) -> Self {
        let cpu_mesh = CpuMesh::new(positions, None);
        Self::new(context, cpu_mesh)
    }

    pub fn _from_positions_and_indices(
        context: &Context,
        positions: Vec<Vec3>,
        indices: Vec<u32>,
    ) -> Self {
        let cpu_mesh = CpuMesh::new(positions, Some(indices));
        Self::new(context, cpu_mesh)
    }

    pub fn _fill(&mut self, cpu_mesh: &CpuMesh) {
        self.cpu_mesh = cpu_mesh.clone();
        self.gpu_mesh.fill(cpu_mesh);
    }

    pub fn embed<F>(&mut self, f: F)
    where
        F: Fn(Vec3) -> Vec3,
    {
        self.cpu_mesh
            .positions
            .iter_mut()
            .for_each(|pos| *pos = f(*pos));
        self.gpu_mesh.positions.fill(&self.cpu_mesh.positions);
    }

    pub fn embed_from<F>(&mut self, mesh: &Mesh, f: F)
    where
        F: Fn(Vec3) -> Vec3,
    {
        self.cpu_mesh.positions = mesh.positions().iter().map(|pos| f(*pos)).collect();
        self.cpu_mesh.indices = mesh.indices().clone();
        self.gpu_mesh.fill(&self.cpu_mesh);
    }

    pub fn embed_from_triangles<F>(&mut self, mesh: &Mesh, f: F)
    where
        F: Fn([Vec3; 3]) -> [Vec3; 3],
    {
        let prev = mesh.positions().clone();
        self.cpu_mesh.positions = prev
            .chunks(3)
            .map(|chunk| {
                f([
                    chunk[0],
                    chunk[1],
                    chunk[2],
                ])
            })
            .flatten()
            .collect();
        self.cpu_mesh.indices = mesh.indices().clone();
        self.gpu_mesh.fill(&self.cpu_mesh);
    }

    pub fn positions(&self) -> &Vec<Vec3> {
        &self.cpu_mesh.positions
    }

    pub fn indices(&self) -> &Vec<u32> {
        &self.cpu_mesh.indices
    }

    pub fn vertex_buffer(&self) -> &VertexBuffer {
        &self.gpu_mesh.positions
    }

    pub fn element_buffer(&self) -> &ElementBuffer {
        &self.gpu_mesh.indices
    }

    pub fn num_vertices(&self) -> usize {
        self.cpu_mesh.positions.len()
    }
}
