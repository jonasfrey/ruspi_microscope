const vertexShaderSource = `#version 300 es
in vec4 a_position;
in vec2 a_texCoord;
out vec2 o_trn_nor_pixel;

void main() {
    gl_Position = a_position;
    o_trn_nor_pixel = a_texCoord;
}
`;

const fragmentShaderSource = `#version 300 es
precision mediump float;

in vec2 o_trn_nor_pixel;
uniform sampler2D u_texture1;
uniform sampler2D u_texture2;
out vec4 fragColor;
uniform vec4 o_mouse;
uniform vec4 o_scl_image1;
uniform vec4 o_scl_image2;

vec4 f_o_convolved_static_3x3(
    vec2 o_trn, 
    float[3*3] a_n_factor_weight_krnl, 
    sampler2D o_texture, 
    vec2 o_scl_o_texture
  ){
  
      vec2 o_scl_krnl = vec2(3.);
      vec2 o_scl_krnl_half = floor(o_scl_krnl/2.);
      
      vec4 o_col_sum = vec4(0.);
      float n_factor_sum = 0.;
      for(float n_x = 0.; n_x < o_scl_krnl.x; n_x+=1.){
          for(float n_y = 0.; n_y < o_scl_krnl.y; n_y+=1.){
              vec2 o_trn_krnl = vec2(n_x, n_y);
              vec2 o_trn_krnl2 = o_trn_krnl-o_scl_krnl_half;
              float n_idx_a_n_krnl = n_y*o_scl_krnl.x + n_x;
              float n_factor = a_n_factor_weight_krnl[int(n_idx_a_n_krnl)];
              n_factor_sum+=n_factor;
              vec4 o_col_tmp =
                  texture(
                    o_texture,
                      (o_trn+o_trn_krnl2.xy)/o_scl_o_texture.xy
                  )*n_factor;
              o_col_sum+= o_col_tmp;
          }
      }
      vec4 o_col_res = o_col_sum;
      if(n_factor_sum != 0.0){
        o_col_res = o_col_res/n_factor_sum;
      }
      return (o_col_res);
  
  
  }
vec4 f_o_sobel_edge_detected(
    vec2 o_trn, 
    sampler2D o_texture, 
    vec2 o_scl_o_texture
){
    vec4 o_col_sobel_x = f_o_convolved_static_3x3(
        o_trn,
        float[](
          //sobel edge detection x
          -1., 0., 1.,
          -2., 0., 2.,
          -1., 0., 1.
        ), 
        o_texture, 
        o_scl_o_texture
    );
    vec4 o_col_sobel_y = f_o_convolved_static_3x3(
        o_trn,
        float[](
          //sobel edge detection y
          1., 2., 1.,
          0., 0., 0.,
          -1., -2., -1.
        ), 
        o_texture, 
        o_scl_o_texture
    );
    vec4 o_col_sobel_x_pow = pow(o_col_sobel_x,vec4(2.));
    vec4 o_col_sobel_y_pow = pow(o_col_sobel_y,vec4(2.));
    return vec4(
      sqrt(o_col_sobel_x_pow.x+o_col_sobel_y_pow.x),
      sqrt(o_col_sobel_x_pow.y+o_col_sobel_y_pow.y),
      sqrt(o_col_sobel_x_pow.z+o_col_sobel_y_pow.z),
      sqrt(o_col_sobel_x_pow.w+o_col_sobel_y_pow.w)
    );


}
void main() {
    ivec2 o_trn = ivec2(o_trn_nor_pixel.xy * o_scl_image1.xy);
    ivec2 o_trn_add = ivec2((o_mouse.xy-.5)*2.*o_scl_image2.xy);
    vec4 color1 = texelFetch(u_texture1, o_trn.xy, 0);
    vec4 o_cs = f_o_sobel_edge_detected(
        vec2(o_trn), 
        u_texture1, 
        o_scl_image1.xy
    );
    vec4 color2 = texelFetch(u_texture2, o_trn.xy+o_trn_add.xy, 0);
    if(color1.xyz == vec3(0.) || color2.xyz == vec3(0.)){
        fragColor = vec4(0.); 
        return;
    }
    // fragColor = vec4(
    //     abs(color1.xyz - color2.xyz),
    //     1.
    // );
    fragColor = vec4(
        (color1.xyz - color2.xyz),
        1.
    );

    fragColor = o_cs;
    //fragColor = mix(color1, color2, 0.5);
    // fragColor = vec4(vec3(o_trn_nor_pixel.x), 1.);
}
`;

function createShader(gl, type, source) {
    const shader = gl.createShader(type);
    gl.shaderSource(shader, source);
    gl.compileShader(shader);
    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
        console.error('Error compiling shader:', gl.getShaderInfoLog(shader));
        gl.deleteShader(shader);
        return null;
    }
    return shader;
}

function createProgram(gl, vertexShader, fragmentShader) {
    const program = gl.createProgram();
    gl.attachShader(program, vertexShader);
    gl.attachShader(program, fragmentShader);
    gl.linkProgram(program);
    if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
        console.error('Error linking program:', gl.getProgramInfoLog(program));
        gl.deleteProgram(program);
        return null;
    }
    return program;
}

function loadImage(url) {
    return new Promise((resolve, reject) => {
        const image = new Image();
        image.src = url;
        image.onload = () => resolve(image);
        image.onerror = (err) => reject(err);
    });
}

function createTexture(gl, image) {
    const texture = gl.createTexture();
    gl.bindTexture(gl.TEXTURE_2D, texture);
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, image);
    gl.generateMipmap(gl.TEXTURE_2D);
    return texture;
}

async function main() {
    const canvas = document.getElementById('glCanvas');
    const gl = canvas.getContext('webgl2');

    if (!gl) {
        console.error('WebGL 2 not available');
        return;
    }

    // Load images
    // const image1 = await loadImage('image1.png');
    // const image2 = await loadImage('image2.png');

    const image1 = await loadImage('1.jpg');
    const image2 = await loadImage('2.jpg');
    let o_scl_image1 = [image1.width, image1.height,0.,0.]
    let o_scl_image2 = [image2.width, image2.height,0.,0.]

    canvas.width = o_scl_image1[0]/20;
    canvas.height = o_scl_image1[1]/20;

    const pixelData = new Uint8Array(canvas.width * canvas.height * 4); // RGBA

    let o_mouse = [0., 0., 0., 0.];

    canvas.onpointermove = function(o_e){
        o_mouse[0] = (o_e.clientX) / canvas?.clientWidth;
        o_mouse[1] = (o_e.clientY) / canvas?.clientHeight;
    }
    canvas.onpointerdown = function(){
        o_mouse[2] = 1.0;
    }
    canvas.onpointerup = function(){
        o_mouse[2] = 0.0;
    }

    gl.viewport(0, 0, canvas.width, canvas.height);

    // Create shaders
    const vertexShader = createShader(gl, gl.VERTEX_SHADER, vertexShaderSource);
    const fragmentShader = createShader(gl, gl.FRAGMENT_SHADER, fragmentShaderSource);

    // Create program
    const program = createProgram(gl, vertexShader, fragmentShader);

    // Look up attribute and uniform locations
    const positionLocation = gl.getAttribLocation(program, 'a_position');
    const texCoordLocation = gl.getAttribLocation(program, 'a_texCoord');
    const texture1Location = gl.getUniformLocation(program, 'u_texture1');
    const texture2Location = gl.getUniformLocation(program, 'u_texture2');

    const o_loc__o_mouse = gl.getUniformLocation(program, 'o_mouse'); 
    const o_loc__o_scl_image1 = gl.getUniformLocation(program, 'o_scl_image1'); 
    const o_loc__o_scl_image2 = gl.getUniformLocation(program, 'o_scl_image2'); 

    // Create buffers and load data into them
    const positionBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
    const positions = [
        -1, -1,
        1, -1,
        -1,  1,
        -1,  1,
        1, -1,
        1,  1,
    ];
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(positions), gl.STATIC_DRAW);

    const texCoordBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, texCoordBuffer);
    const texCoords = [
        0, 0,
        1, 0,
        0, 1,
        0, 1,
        1, 0,
        1, 1,
    ];
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(texCoords), gl.STATIC_DRAW);

    // Create textures
    const texture1 = createTexture(gl, image1);
    const texture2 = createTexture(gl, image2);

    // Tell WebGL to use our program (pair of shaders)
    gl.useProgram(program);

    // Bind the position buffer
    gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
    gl.enableVertexAttribArray(positionLocation);
    gl.vertexAttribPointer(positionLocation, 2, gl.FLOAT, false, 0, 0);

    // Bind the texCoord buffer
    gl.bindBuffer(gl.ARRAY_BUFFER, texCoordBuffer);
    gl.enableVertexAttribArray(texCoordLocation);
    gl.vertexAttribPointer(texCoordLocation, 2, gl.FLOAT, false, 0, 0);

    // Set the textures
    gl.activeTexture(gl.TEXTURE0);
    gl.bindTexture(gl.TEXTURE_2D, texture1);
    gl.uniform1i(texture1Location, 0);

    gl.activeTexture(gl.TEXTURE1);
    gl.bindTexture(gl.TEXTURE_2D, texture2);
    gl.uniform1i(texture2Location, 1);

    // Draw the rectangle
    gl.clearColor(0, 0, 0, 0);
    gl.clear(gl.COLOR_BUFFER_BIT);
    gl.drawArrays(gl.TRIANGLES, 0, 6);


    gl.uniform4f(o_loc__o_scl_image1, o_scl_image1[0], o_scl_image1[1], o_scl_image1[2],o_scl_image1[3]);
    gl.uniform4f(o_loc__o_scl_image2, o_scl_image2[0], o_scl_image2[1], o_scl_image2[2],o_scl_image2[3]);



    function render() {
        // Calculate normalized mouse coordinates and pass them to the shader

        gl.uniform4f(o_loc__o_mouse, o_mouse[0], o_mouse[1], o_mouse[2],o_mouse[3]);

        // Draw the rectangle
        gl.clearColor(0, 0, 0, 0);
        gl.clear(gl.COLOR_BUFFER_BIT);
        gl.drawArrays(gl.TRIANGLES, 0, 6);


        // Read the pixels from the framebuffer
        gl.readPixels(0, 0, canvas?.width, canvas.height, gl.RGBA, gl.UNSIGNED_BYTE, pixelData);

        // Sum all pixel values
        let sum = 0;
        let n_count = 0;
        for (let i = 0; i < pixelData.length; i += 4) {
            if(pixelData[i+3]!= 0){

                sum += pixelData[i] + pixelData[i + 1] + pixelData[i + 2] + pixelData[i + 3];
                n_count +=1;
            }
        }
        let n_avg_sum = sum / n_count;
        console.log('avg Sum of all pixels:', n_avg_sum);

        requestAnimationFrame(render);
    }

    // Start the render loop
    render();

}

main();
