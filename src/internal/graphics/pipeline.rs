pub trait RenderPipeline {
    fn render(&self, render_pass: &mut wgpu::RenderPass);
}
