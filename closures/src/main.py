from time import sleep
from typing import Callable

class Cacher:
    def __init__(self, calculation: Callable) -> None:
        self.calculation = calculation
        self.hash_map = {}

    def value(self, arg):
        if arg in self.hash_map.keys():
            return self.hash_map[arg]
        else:
            calc = (self.calculation)(arg)
            self.hash_map[arg] = calc
            return calc

def cacher_fun(num):
    print("Calculating slowly...")
    sleep(2)
    return num + 5

def generate_workout(intensity: int, rand_num: int):
    cacher = Cacher(cacher_fun)

    print(f"Intensity: {intensity}")
    print(f"Random number: {rand_num}\n\n")

    print(f"FirstCalc: {cacher.value(1)}\n")
    print(f"SecondtCalc: {cacher.value(1)}\n")
    print(f"ThirdCalc: {cacher.value(5)}\n")

def main():
    simulated_intensity = 10
    simulated_random_number = 7

    generate_workout(simulated_intensity, simulated_random_number)

if __name__ == "__main__":
    main()