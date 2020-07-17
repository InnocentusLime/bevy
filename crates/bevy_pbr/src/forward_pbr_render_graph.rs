use crate::{
    material::StandardMaterial,
    nodes::LightsNode,
    pipelines::{build_forward_pipeline, FORWARD_PIPELINE_HANDLE},
};
use bevy_asset::Assets;
use bevy_ecs::Resources;
use bevy_render::{
    pipeline::PipelineDescriptor,
    render_graph::{base, AssetRenderResourcesNode, RenderGraph, RenderResourcesNode},
    shader::Shader,
};
use bevy_transform::prelude::Transform;

pub mod node {
    pub const TRANSFORM: &str = "transform";
    pub const STANDARD_MATERIAL: &str = "standard_material";
    pub const LIGHTS: &str = "lights";
}

pub mod uniform {
    pub const LIGHTS: &str = "Lights";
}

pub trait ForwardPbrRenderGraphBuilder {
    fn add_pbr_graph(&mut self, resources: &Resources) -> &mut Self;
}

impl ForwardPbrRenderGraphBuilder for RenderGraph {
    fn add_pbr_graph(&mut self, resources: &Resources) -> &mut Self {
        self.add_system_node(node::TRANSFORM, RenderResourcesNode::<Transform>::new(true));
        self.add_system_node(
            node::STANDARD_MATERIAL,
            AssetRenderResourcesNode::<StandardMaterial>::new(true),
        );
        self.add_system_node(node::LIGHTS, LightsNode::new(10));
        let mut shaders = resources.get_mut::<Assets<Shader>>().unwrap();
        let mut pipelines = resources.get_mut::<Assets<PipelineDescriptor>>().unwrap();
        pipelines.set(
            FORWARD_PIPELINE_HANDLE,
            build_forward_pipeline(&mut shaders),
        );

        // TODO: replace these with "autowire" groups
        self.add_node_edge(node::STANDARD_MATERIAL, base::node::MAIN_PASS)
            .unwrap();
        self.add_node_edge(node::TRANSFORM, base::node::MAIN_PASS)
            .unwrap();
        self.add_node_edge(node::LIGHTS, base::node::MAIN_PASS)
            .unwrap();
        self
    }
}
