# sudo apt-get install python3-rpi.gpio
import RPi.GPIO as GPIO
import time
import math
# Set the GPIO pin number
pwm_pin = 18

# Set the GPIO mode (BCM is the Broadcom pin numbering)
GPIO.setmode(GPIO.BCM)

# Set up the pin as an output
GPIO.setup(pwm_pin, GPIO.OUT)

# Set the PWM frequency (1 kHz)
n_ms = 3
frequency = 1000/n_ms
# Initialize PWM with a 50% duty cycle
pwm = GPIO.PWM(pwm_pin, frequency)
pwm.start(50)

n = 0
while(n < 100000):
    n+=0.005
    # n2 = n%100
    n2 = (math.sin(n)*.5+.5)*100
    print(n2)
    pwm.ChangeDutyCycle(n2)#
    time.sleep(0.001)

# Change the frequency to 500 Hz after 4 seconds
pwm.ChangeFrequency(500)

# Stop PWM and clean up
pwm.stop()
GPIO.cleanup()