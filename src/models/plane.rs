use crate::engine::model::Model;
use rand::random;

pub struct Plane {
    size: u32,
    normals: (),
    vertices: Vec<f32>,
    indices: Vec<u32>
}

impl Plane {
    pub fn new(size: u32) -> Plane {
        let num_vertices = (size + 1) * (size + 1);
        let num_faces = size * size;

        let mut plane = Plane {
            size,
            normals: (),
            vertices: Vec::with_capacity((num_vertices * 3) as usize),
            indices: Vec::with_capacity((num_faces * 6) as usize)
        };

        // Generate all the vertices
        for index in 0..num_vertices {
            let vertex = plane.index_to_vertex(index);
            plane.vertices.push(vertex.0 as f32); // x
            plane.vertices.push(random::<f32>()); // y
            plane.vertices.push(vertex.1 as f32); // z
        }

        // Generate the indices
        for index in 0..num_faces {
            let face = plane.index_to_face(index);

            // Calculate all the indices
            let tl = plane.vertex_to_index(face.0,     face.1);
            let tr = plane.vertex_to_index(face.0 + 1, face.1);
            let bl = plane.vertex_to_index(face.0,     face.1 + 1);
            let br = plane.vertex_to_index(face.0 + 1, face.1 + 1);

            // First triangle
            plane.indices.push(tl);
            plane.indices.push(bl);
            plane.indices.push(tr);

            plane.indices.push(bl);
            plane.indices.push(br);
            plane.indices.push(tr);

            // TODO: Calculate normals for each face
        }
        
        plane
    }

    pub fn to_model(&self, shader: usize) -> Model {
        Model::new(&self.vertices, &self.indices, shader)
    }

    fn vertex_to_index(&self, x: u32, y: u32) -> u32 {
        (y * (self.size + 1)) + x
    }

    fn face_to_index(&self, x: u32, y: u32) -> u32 {
        (y * self.size) + x
    }

    fn index_to_vertex(&self, index: u32) -> (u32, u32) {
        (index % (self.size + 1), index / (self.size + 1))
    }

    fn index_to_face(&self, index: u32) -> (u32, u32) {
        (index % self.size, index / self.size)
    }
}