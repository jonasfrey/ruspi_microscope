const port = 8081;

const handler = async (request) => {
    let o_url = new URL(request.url);
    let s_name_file = o_url.pathname.split('/').pop();
    // console.log(s_name_file)
    if(s_name_file.trim() == ''){
        s_name_file = './read_usb_camera.html'
    }

    let o_s_ext_s_mime = {
        'js': "text/javascript", 
        'html': "text/html"
    }
    let s_mime = o_s_ext_s_mime[
        s_name_file.split('.').pop()
    ];
    if(!s_mime){
        s_mime = 'text/plain'
    }
  return new Response(
        await Deno.readTextFile(s_name_file),
        { 
            headers: 
                { 
                    'content-type': `${s_mime}`
                },
            status: 200
        }
    
    );
};

console.log(`HTTP server running. Access it at: http://localhost:8080/`);
Deno.serve({ port }, handler);