i = input("Enter when or how long for alarm (in HH:MM:SS or HH:MM)\n")

import time, datetime;
current = time.time()

def convertToSec(string):
    coef = 1
    secs = 0
    ms = string.split(':')
    ms.reverse()
    for x in ms:
        secs += int(x) * coef
        coef *= 60
    return secs

end = 0
if i.find('in ') == 0:
    end = current + convertToSec(i[3:])
else:
    today = datetime.date.today()
    t = datetime.datetime.strptime(i, "%H:%M").time()
    end = time.mktime(datetime.datetime.combine(today, t).timetuple())

scale = (end - time.time()) / 10
print("Waiting for", scale * 10, "seconds")
for x in range(10):
    time.sleep(scale)
    print("{0:.2f}".format(time.time() - current), "seconds passed...")
print("time up!")
