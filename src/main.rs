mod engine;

use engine::interface as I;
use engine::impls::gl;



struct MainPass{

}

impl<R: I::Renderer> I::Pass<MyContext, R> for MainPass {
    fn getPrerequisites(&self) -> Vec<Box<I::Pass<MyContext, R>>> {
        Vec::new()
    }
    fn needRendering(&self, ctx: &mut MyContext) -> bool {true}
    fn render(&self, r: &R, ctx: &mut MyContext) {
        
        println!("mainPass rendered")
    }
}



struct MyContext{
    frameCounter: usize
}

impl I::Context for MyContext {
    fn increaseCounter(&mut self) -> usize {
        self.frameCounter += 1;
        self.frameCounter
    }
}


fn main() {
    let pass = MainPass{};
    let gl = gl::createRenderer();
    let mut ctx = MyContext{frameCounter: 0};
    I::render(&gl, &pass, &mut ctx)

}
