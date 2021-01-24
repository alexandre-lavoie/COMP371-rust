use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

pub fn link_program_str(
    gl: &WebGl2RenderingContext,
    vertex_shader: &str,
    fragment_shader: &str,
) -> WebGlProgram {
    let v_result = compile_shader(gl, WebGl2RenderingContext::VERTEX_SHADER, vertex_shader);

    let vertex_shader = match v_result {
        Ok(v) => v,
        Err(s) => panic!("{:?}", s),
    };

    let f_result = compile_shader(
        &gl,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        fragment_shader,
    );

    let fragment_shader = match f_result {
        Ok(f) => f,
        Err(s) => panic!("{:?}", s),
    };

    match link_program(&gl, &vertex_shader, &fragment_shader) {
        Ok(p) => p,
        Err(s) => panic!("{:?}", s),
    }
}
