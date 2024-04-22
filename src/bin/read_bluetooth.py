# pip install pybluez
import bluetooth

target_name = "My Device"
target_address = None

nearby_devices = bluetooth.discover_devices()

for addr in nearby_devices:
    if target_name == bluetooth.lookup_name(addr):
        target_address = addr
        break

if target_address is not None:
    print(f"Found target Bluetooth device with address: {target_address}")
    port = 1  # Bluetooth port, may need to be adjusted
    sock = bluetooth.BluetoothSocket(bluetooth.RFCOMM)
    sock.connect((target_address, port))
    data = sock.recv(1024)  # Receive up to 1024 bytes
    print("Received:", data)
    sock.close()
else:
    print("Could not find target Bluetooth device nearby")
