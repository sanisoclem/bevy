use bevy::prelude::*;
use bevy_render::{mesh::{VertexAttributeValues, VertexAttribute}, pipeline::PrimitiveTopology};

pub fn get_hex_vertices(center: Vec3, normal: Vec3, up: Vec3, size: f32) -> Vec<Vec3> {
    // `up` and `normal` should both be normalized and perpedicular (cross product should also be normalized)
    // get the a vec perpendicular to both up and normal
    let start = up.cross(normal);

    // rotate 60deg 6 times along the normal to correspond to each vertex of hex
    (0..6).map(|rot| (rot as f32 * 60.0).to_radians())
    .map(|angle| Quat::from_axis_angle(normal, angle))
    .map(|quat| (quat.mul_vec3(start) * size) + center)
    .collect()
}

pub fn mesh_hex_outline(center: Vec3, normal: Vec3, up: Vec3, size: f32) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::LineList);

    // compute vertices
    let vertices= get_hex_vertices(center, normal, up, size);
    mesh.attributes.push(VertexAttribute {
        name: "Vertex_Position".into(),
        values: VertexAttributeValues::Float3(vertices.iter().map(|v| [v.x(), v.y(), v.z()]).collect()),
    });

    // compute normals
    mesh.attributes.push(VertexAttribute {
        name: "Vertex_Normal".into(),
        values: VertexAttributeValues::Float3(vertices.iter().map(|_| [normal.x(),normal.y(),normal.z()]).collect()),
    });

    // compute UVs
    mesh.attributes.push(VertexAttribute {
        name: "Vertex_Uv".into(),
        values: VertexAttributeValues::Float2(vertices.iter().map(|_| [0.0, 0.0]).collect()),
    });

    // indices
    mesh.indices = Some(vec![0,1,1,2,2,3,3,4,4,5,5,0]);
    mesh
}

pub fn mesh_hex_plane(center: Vec3, normal: Vec3, up: Vec3, size: f32) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    // compute vertices
    let vertices= get_hex_vertices(center, normal, up, size);
    mesh.attributes.push(VertexAttribute {
        name: "Vertex_Position".into(),
        values: VertexAttributeValues::Float3(vertices.iter().map(|v| [v.x(), v.y(), v.z()]).collect()),
    });

    // compute normals
    mesh.attributes.push(VertexAttribute {
        name: "Vertex_Normal".into(),
        values: VertexAttributeValues::Float3(vertices.iter().map(|_| [normal.x(),normal.y(),normal.z()]).collect()),
    });

    // compute UVs
    mesh.attributes.push(VertexAttribute {
        name: "Vertex_Uv".into(),
        values: VertexAttributeValues::Float2(vertices.iter().map(|_| [0.0, 0.0]).collect()),
    });

    // indices
    mesh.indices = Some(vec![5,0,1,2,3,4,5,1,2,2,4,5]);
    mesh
}

pub fn mesh_hex_voxel(top: Vec3, bottom: Vec3, normal: Vec3, up: Vec3, size: f32) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    // compute vertices
    let vertices_top= get_hex_vertices(top, normal, up, size);
    let vertices_bot= get_hex_vertices(bottom, normal, up, size);
    mesh.attributes.push(VertexAttribute {
        name: "Vertex_Position".into(),
        values: VertexAttributeValues::Float3(vertices_top.iter().chain(vertices_bot.iter()).map(|v| [v.x(), v.y(), v.z()]).collect()),
    });

    // compute normals
    mesh.attributes.push(VertexAttribute {
        name: "Vertex_Normal".into(),
        values: VertexAttributeValues::Float3(vertices_top.iter().map(|_| [normal.x(),normal.y(),normal.z()]).chain(vertices_bot.iter().map(|_| [-normal.x(),-normal.y(),-normal.z()])).collect()),
    });

    // compute UVs
    mesh.attributes.push(VertexAttribute {
        name: "Vertex_Uv".into(),
        values: VertexAttributeValues::Float2(vertices_top.iter().chain(vertices_bot.iter()).map(|_| [0.0, 0.0]).collect()),
    });

    // indices
    mesh.indices = Some(vec!
        [ 5, 0, 1
        , 2, 3, 4
        , 5, 1, 2
        , 2, 4, 5

        , 5, 11, 6
        , 0, 5, 6
        , 0, 6, 7
        , 1, 0, 7
        , 1, 7, 8
        , 2, 1, 8
        , 2, 8, 9
        , 3, 2, 9
        , 3, 9, 10
        , 4, 3, 10
        , 4, 10, 11
        , 5, 4, 11 ]);
    mesh
}