use crate::{
	asset::AssetRef,
	gpu,
	graphics,
	math,
	Viewport,
	ViewportId,
};

use gpu::{
	Gpu,
	Pipeline,
};
use graphics::Mesh;

use math::Matrix4;
use newport_engine::Engine;

use std::collections::HashMap;

pub struct RenderState {
	pub viewports: HashMap<ViewportId, Viewport>,

	pub primitives: Vec<Primitive>,
	pub primitive_transforms: Vec<Matrix4>,
}

impl RenderState {
	pub fn render(&self) -> HashMap<ViewportId, gpu::Texture> {
		let engine = Engine::as_ref();
		let gpu: &Gpu = engine.module().unwrap();
		let device = gpu.device();

		let transform_buffer = device
			.create_buffer(
				gpu::BufferUsage::CONSTANTS,
				gpu::MemoryType::HostVisible,
				self.primitive_transforms.len(),
			)
			.unwrap();
		transform_buffer.copy_to(&self.primitive_transforms[..]);

		let view_matrices: Vec<Matrix4> = self
			.viewports
			.iter()
			.map(|(_id, viewport)| {
				let aspect_ratio = viewport.width as f32 / viewport.height as f32;
				let projection = Matrix4::perspective(viewport.fov, aspect_ratio, 10000.0, 0.1);
				// TODO: View matrix
				projection
			})
			.collect();

		let views_buffer = device
			.create_buffer(
				gpu::BufferUsage::CONSTANTS,
				gpu::MemoryType::HostVisible,
				view_matrices.len(),
			)
			.unwrap();
		views_buffer.copy_to(&view_matrices[..]);

		let mut result = HashMap::with_capacity(self.viewports.len());
		self.viewports.iter().for_each(|(id, viewport)| {
			let backbuffer = device
				.create_texture(
					gpu::TextureUsage::SAMPLED | gpu::TextureUsage::COLOR_ATTACHMENT,
					gpu::MemoryType::DeviceLocal,
					gpu::Format::RGBA_U8,
					viewport.width,
					viewport.height,
					1,
				)
				.unwrap();

			result.insert(*id, backbuffer);
		});

		result
	}
}

pub enum Primitive {
	StaticMesh {
		mesh: AssetRef<Mesh>,
		pipeline: AssetRef<Pipeline>,
	},
}
