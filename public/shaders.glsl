

vec4 f_o_convolved_static_3x3(
    vec2 o_trn, 
    float[3*3] a_n_factor_weight_krnl, 
    float[3*3] a_n_exponent_weight_krnl
){

    vec2 o_scl_krnl = vec2(3.);
    vec2 o_scl_krnl_half = floor(o_scl_krnl/2.);
    
    
    vec4 o_col_sum = vec4(0.);
        
    for(float n_x = 0.; n_x < o_scl_krnl.x; n_x+=1.){
        for(float n_y = 0.; n_y < o_scl_krnl.y; n_y+=1.){
            vec2 o_trn_krnl = vec2(n_x, n_y);
            vec2 o_trn_krnl2 = o_trn_krnl-o_scl_krnl_half;
            float n_idx_a_n_krnl = n_y*o_scl_krnl.x + n_x;
            float n_factor = a_n_factor_weight_krnl[int(n_idx_a_n_krnl)];
            float n_exponent = a_n_exponent_weight_krnl[int(n_idx_a_n_krnl)];
            
            vec4 o_col_tmp =
                texture(
                    iChannel0,
                    (o_trn+o_trn_krnl2.xy)/iResolution.xy
                )*n_factor;
            //o_col_tmp = pow(o_col_tmp, vec4(n_exponent));
            o_col_sum+= o_col_tmp;
        }
    }
    vec4 o_col_res = o_col_sum;

    return (o_col_res);
}
void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
    vec2 o_trn = fragCoord.xy;
    float n_pixelation = 1.;
    o_trn=floor(o_trn*(1./n_pixelation))*n_pixelation;
    vec2 o_trn_nor = fragCoord.xy/iResolution.xy;
    
    vec4 o_col_img = texture(iChannel0,(o_trn)/iResolution.xy);
    float n = 1./(3.*3.);

        vec4 o_col_sobel_x = f_o_convolved_static_3x3(
            o_trn,
            float[](
            //sobel edge detection x
            -1., 0., 1.,
            -2., 0., 2.,
            -1., 0., 1.
            ),
            float[](
            1., 1., 1.,
            1., 1., 1.,
            1., 1., 1.
            )
        );
        vec4 o_col_sobel_y = f_o_convolved_static_3x3(
            o_trn,
            float[](
            //sobel edge detection y
            1., 2., 1.,
            0., 0., 0.,
            -1., -2., -1.
            ),
            float[](
            1., 1., 1.,
            1., 1., 1.,
            1., 1., 1.
            )
        );
        
    vec4[] a_o_col = vec4[](
        o_col_img,
        f_o_convolved_static_3x3(
            o_trn,
            float[](
            //slight blur
            1./16., 1./8., 1./16.,
            1./8., 1./4., 1./8., 
            1./16., 1./8., 1./16. 
            ),
            float[](
            1.0, 1.0, 1.0,
            1.0, 1.0, 1.0,
            1.0, 1.0, 1.0
            )
        ), 
        f_o_convolved_static_3x3(
            o_trn,
            float[](
            //slight 'de'blur
            -1./16., -1./8., -1./16.,
            -1./8., 1., -1./8., 
            -1./16., -1./8., -1./16. 
            ),
            float[](
            1.0, 1.0, 1.0,
            1.0, 1.0, 1.0,
            1.0, 1.0, 1.0
            )
        ), 
        normalize(vec4(o_col_sobel_x.x)),
        normalize(vec4(o_col_sobel_y.x)),
            //sobel edge x+y
        normalize(
            (o_col_sobel_x+o_col_sobel_y)
        ),
        f_o_convolved_static_3x3(
            o_trn,
            float[](
            0.0, 0.0, 0.0,
            0.5, -0.2, 0.0,
            0.0, 0.0, 0.0
            ),
            float[](
            1.0, 1.0, 1.0,
            1.0, 1.0, 1.0,
            1.0, 1.0, 1.0
            )
        )
        
    );
   
    float n_idx = (o_trn_nor.x*float(a_o_col.length()));
    float n_idx_floor = floor(n_idx);
    
    float n_line = smoothstep(0.02, 0.0, fract(n_idx));
    fragColor = (a_o_col[int(n_idx)])+vec4(n_line);

    
}