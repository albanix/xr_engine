use ogl33::{GL_ARRAY_BUFFER, GL_COLOR_BUFFER_BIT, GL_FALSE, GL_FLOAT, GL_STATIC_DRAW, GL_VERTEX_SHADER, glBindBuffer, glBindVertexArray, glBufferData, glClear, glClearColor, glCreateShader, glEnableVertexAttribArray, glGenBuffers, glGenVertexArrays, glShaderSource, glVertexAttribPointer};
use super::{
    shader::{
        FRAGMENT_SHADERL,
        SHADER
    },
    gl::ObjectGL
};

pub struct Triangle;

impl Triangle {
    pub fn draw(&self, verites: [[f32;3];3]) {
        let mut obj = ObjectGL {
            vao: 0,
            vbo: 0
        };

        unsafe {
            // Спочатку нам потрібно очситити екран
            glClearColor(0.2, 0.3, 0.3, 1.0);
            glClear(GL_COLOR_BUFFER_BIT);

            // VAO формує таку, так би мовити структуру данних, як читати стан, тобто весь перелік нащших команд opengl
            glGenVertexArrays(1, &mut obj.vao);
            assert_ne!(obj.vao, 0);

            // assert_ne! - макрос для швидкої перевірки збіжности умови

            
            glGenBuffers(1, &mut obj.vbo);
            assert_ne!(obj.vbo, 0);

            glBindBuffer(GL_ARRAY_BUFFER, obj.vbo);
            glBindVertexArray(obj.vao);
            
            glBufferData(
                GL_ARRAY_BUFFER,
                size_of_val(&verites) as isize, 
                verites.as_ptr().cast(),
                GL_STATIC_DRAW
            );
            
            glVertexAttribPointer(
                0,
                3,
                GL_FLOAT,
                GL_FALSE,
                size_of::<[[f32;3];3]>().try_into().unwrap(),
                0 as *const _
            );

            glEnableVertexAttribArray(0);

            let vertex_shader = glCreateShader(GL_VERTEX_SHADER);
            assert_ne!(vertex_shader, 0);

            glShaderSource(
                vertex_shader, 
                1,
                &(SHADER.as_bytes().as_ptr().cast()),
                &(SHADER.len().try_into().unwrap())
            );
        }
    }
}