pub enum IndexType {
    size8,
    size16,
    size32
}

pub enum GeometryRenderMode {
    Triangles,
    TriangleStrip,
    TriangleFan,
    Points,
    Lines
}

pub trait RenderableGeometry{
    fn index(&self) -> Option<Vec<i32>>;
    fn positions(&self) -> &Vec<f32>;
    fn count(&self) -> usize;
    fn itemSize(&self) -> IndexType; 
    fn renderMode(&self) -> GeometryRenderMode;

}

pub trait Pass<C, R:Renderer> {
    fn getPrerequisites(&self) -> Vec<Box<Pass<C, R>>>;
    fn needRendering(&self, ctx: &mut C) -> bool;
    fn render(&self, r: &R, ctx: &mut C);
}

pub trait EncoderTrait {
    type Buffer;
    // type Texture;
    fn encodeBuffer<I>(&mut self, buf: &Vec<I>) -> Self::Buffer;
    fn dropBuffer(&mut self, buf: &mut Self::Buffer);
    
}

pub trait Renderer {
    type Geometry;
    type Encoder: EncoderTrait;
    fn getEncoder(&mut self) -> &mut Self::Encoder;
    fn renderGeometry(&self, geometry: &Self::Geometry);
    fn createGeometry(&mut self, geom: &RenderableGeometry) -> Self::Geometry;
}

pub trait Context{
    fn increaseCounter(&mut self) -> usize;
}

pub fn render<R:Renderer, P:Pass<C, R>, C:Context> (r: &R, p: &P, ctx: &mut C) {
    ctx.increaseCounter();
    let prerequisites = p.getPrerequisites();
    for pass in prerequisites {
        if pass.needRendering(ctx) {
            pass.render(r, ctx);
        }
    }

    p.render(r, ctx)

}

