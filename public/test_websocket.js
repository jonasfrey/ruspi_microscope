// Replace 'ws://example.com/socket' with the URL of your WebSocket server
const o_ws = new WebSocket(`ws://${location.hostname}:${location.port}/ws`);

// Connection opened
o_ws.addEventListener('open', async function (event) {
    console.log("WebSocket is open now.");

    o_ws.send(JSON.stringify({
        s_command_to_run: 'lsusb'
    }))

    o_ws.send(JSON.stringify({
        s_command_to_run: 'lsblk'
    }))

    o_ws.send(JSON.stringify({
        s_command_to_run: 'touch lol_test'
    }))


    var [n_id_vendor, n_id_product] = "Bus 001 Device 002: ID 046d:c31c Logitech, Inc. Keyboard K120".split(' ')[5].split(':').map(s=>parseInt(`0x${s}`))
    o_ws.send(JSON.stringify({
        s_name_function: 'switch_usb_device', 
        n_id_product,
        n_id_vendor
    }))
    await new Promise((f_res)=>{setTimeout(()=>{return f_res(true)},10000)})
    console.log('hasdf')
    var [n_id_vendor, n_id_product] = "Bus 003 Device 007: ID 045e:028e Microsoft Corp. Xbox360 Controller".split(' ')[5].split(':').map(s=>parseInt(`0x${s}`))
    o_ws.send(JSON.stringify({
        s_name_function: 'switch_usb_device', 
        n_id_product,
        n_id_vendor
    }))
    

});

// Listen for messages
o_ws.addEventListener('message', function (event) {
  let v_o = null;
  try {
    v_o = JSON.parse(event.data);
    
  } catch (error) {
    
  }
  console.log(event.data)
  console.log("message received")
});

// Listen for possible errors
o_ws.addEventListener('error', function (event) {
    console.log('WebSocket error: ', event);
});

// Listen for when the socket closes
o_ws.addEventListener('close', function (event) {
    console.log('WebSocket is closed now.');
});

