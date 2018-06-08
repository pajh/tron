import os

if not os.path.isfile("./current.txt"):
    print("ERROR: No current.txt file")
    exit()

if os.path.isfile("./append.txt"):
    os.remove("./append.txt")

vno = 1
while True:
    fname = "train" + str(vno).zfill(3) + ".txt"
    if not os.path.isfile("./" + fname):
        break
    vno = vno + 1
cmd = "ren current.txt " + fname
print(cmd)
os.system(cmd)
cmd = "copy /b "
for i in range(1,vno+1):
    nm = "train" + str(i).zfill(3) + ".txt "
    if i > 1:
        cmd = cmd + "+ "
    cmd = cmd + nm
cmd = cmd + "append.txt"
print(cmd)
os.system(cmd)

