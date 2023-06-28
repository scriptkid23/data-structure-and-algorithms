import random
import datetime 

date = datetime.datetime.now().strftime("%Y-%m-%dT%H:%M:%S.%f")
arr = ""
fo = open(date+".txt", "w")

for i in range(0, 100000):
   
    arr = arr+str( random.randint(0,i))+" "

fo.write(arr)