import csv

#clear q.csv and write header
with open("q.csv", "w") as f:
    f.write("Question,Answer1,Answer2,Answer3,Answer4,Correct Answer\n")

with open("q.txt") as f:
    lines = f.readlines()
    for (line_number, line) in enumerate(lines):
        line.strip()
        line.replace("\n", "")
        if not line.startswith("答："):
            print(line_number+1, line)
            exit()
        line = line[2:]
        answer = int(line[0])
        if answer < 1 or answer > 4:
            print(line_number+1, line)
            exit()
        if not " _" in line:
            print(line_number+1, line)
            exit()
        line = line[3:]
        if not "(1)" in line and not "(2)" in line and not "(3)" in line and not "(4)" in line:
            print("NOT ALL ANSWERS")
            print(line_number+1, line)
            exit()
        pos1 = line.find("(1)")
        pos2 = line.find("(2)")
        pos3 = line.find("(3)")
        pos4 = line.find("(4)")
        # check the order of the answers
        if pos1 > pos2 or pos2 > pos3 or pos3 > pos4:
            print("WRONG ORDER")
            print(line_number+1, line)
            exit()
        question = line.split("(1)")[0].strip()
        answer1 = line.split("(1)")[1].split("(2)")[0].strip()
        answer2 = line.split("(2)")[1].split("(3)")[0].strip()
        answer3 = line.split("(3)")[1].split("(4)")[0].strip()
        answer4 = line.split("(4)")[1].strip()
        answer4 = answer4.replace("\n", "").strip()
        # write line to csv file
        with open("q.csv", "a") as f:
            writer = csv.writer(f)
            writer.writerow([question, answer1, answer2, answer3, answer4, answer])

print("DONE")