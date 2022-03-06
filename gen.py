with open("dictionary.txt", 'r') as input:
    with open("output.txt", 'w') as output:
        words = input.readlines()
        for data in words:
            data = data.strip()
            data = data.split(' ')
            word = data[0]
            freq = data[1]
            entry = '\t' + '"' + word + '" ' + "=> " + str(freq) + ",\n"
            output.write(entry)
