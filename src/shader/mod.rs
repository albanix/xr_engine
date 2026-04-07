pub const SHADER: &str = r#"version 330 core
    layout (location = 0) in vec3 Pos;

    void main() {
        gl_Positon = vec4(pos.xyz, 1.0);
    }
"#;

pub const FRAGMENT_SHADERL: &str = r#"version 330 core
    out vec4 final_color;
    

    void main() {
        final_color = vec4(1.0, 0.5, 0.2, 1.0);
    }
"#;