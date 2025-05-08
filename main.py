scores = [2,6,5,5,6,4,7,8,9,5,0]
f = open("CsScores.txt", "w")
for index in range(10):
    f.write(str(scores[index]) + "\n")

f.close()
