# Process Abstraction
def calculate_hp(torque, rpm):
    return (torque * rpm) / 5252


hp = calculate_hp(500, 10000)

print ("Horse Power calculated with process abstraction:", hp)




# Data Abstraction
class Car:
    def __init__(self, torque, rpm):
        self.torque = torque
        self.rpm = rpm


    def horsePower(self):
        return (self.torque * self.rpm) / 5252


hp = Car(500, 10000)
print("Horse Power calculated with data abstraction:", hp.horsePower())
