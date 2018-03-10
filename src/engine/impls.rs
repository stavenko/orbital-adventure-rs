pub mod gl {
    extern crate gl;
    extern crate glutin;
    // use std::rc::Rc;
    use engine::interface as I;
    use engine::interface::EncoderTrait;
    pub struct GLRenderer {
        vertexArrays: Vec<usize>,
        encoder: GLEncoder
    }

    enum BufferEnum {
        ArrayBuffer
    }

    pub struct GLBuffer {
        id: usize,
        tp: BufferEnum,
        isDeleted: bool
    }


    pub struct GLGeometry {
        buffers: Vec<GLBuffer>,
        index: Option<GLBuffer>,
        mode: I::GeometryRenderMode,
        count: usize,
        tp: I::IndexType
    }

    pub struct GLEncoder {
        buffers: Vec<usize>
    }

    impl I::EncoderTrait for GLEncoder {
        type Buffer = GLBuffer;

        fn dropBuffer(&mut self, buf: &mut GLBuffer) {
            
        }

        fn encodeBuffer<I>(&mut self, buf:& Vec<I>) -> GLBuffer {
            GLBuffer{id:0, tp: BufferEnum::ArrayBuffer, isDeleted: false}
        }
    }

    impl I::Renderer for GLRenderer {
        type Geometry = GLGeometry;
        type Encoder = GLEncoder;

        fn getEncoder(&mut self) -> &mut GLEncoder{
            &mut self.encoder
        }

        fn createGeometry(&mut self, geom: &I::RenderableGeometry) -> GLGeometry {
            let encoder = self.getEncoder();
            let index: Option<GLBuffer> = match geom.index() {
                Some(array) => Some(encoder.encodeBuffer(&array)),
                None => None
            };
            let mut buffers = Vec::new();
            buffers.push(encoder.encodeBuffer(&geom.positions()));

            GLGeometry{
                index, 
                buffers, 
                mode: geom.renderMode(), 
                count: geom.count(),
                tp: geom.itemSize()
            }
        }

        fn renderGeometry(&self, geometry: &GLGeometry) {
            println!("I rendering geometry, I Swear");
        }
    }

    pub fn createRenderer() -> GLRenderer {
        let encoder = GLEncoder{buffers: vec![]};

        GLRenderer {encoder, vertexArrays: vec![]}
    }
}
