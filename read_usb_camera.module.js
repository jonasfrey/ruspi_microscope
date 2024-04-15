let o_el_vid = document.querySelector('video');
if(!o_el_vid){
    o_el_vid = document.createElement('video');
    document.body.appendChild(o_el_vid)
}

async function startWebcam() {
  try {
    const stream = await navigator.mediaDevices.getUserMedia({
      video: { width: 1920, height: 1080 }
    });
    o_el_vid.srcObject = stream;
  } catch (error) {
    console.error('Error accessing the webcam:', error);
  }
}

// Replace 'ws://example.com/socket' with the URL of your WebSocket server
const socket = new WebSocket('ws://localhost:9000');
window.o_socket = socket
// Connection opened
socket.addEventListener('open', function (event) {
    console.log("WebSocket is open now.");
});

// Listen for messages
socket.addEventListener('message', function (event) {
    console.log('Message from server ', event.data);
});

// Listen for possible errors
socket.addEventListener('error', function (event) {
    console.log('WebSocket error: ', event);
});

// Listen for when the socket closes
socket.addEventListener('close', function (event) {
    console.log('WebSocket is closed now.');
});



startWebcam();