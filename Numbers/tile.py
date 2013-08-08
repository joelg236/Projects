print("What is the width?")
width = eval(raw_input())

print("What is the height?")
height = eval(raw_input())

print("What is the cost per tile?")
tile = eval(raw_input())

print("\nTotal cost: "+ str((width * height) / tile))
