with open("dictionary.txt", 'r') as dictionary:
    dictionary = dictionary.readlines()
    dictionary = [d.rstrip() for d in dictionary]

with open("answers.txt", 'r') as answers:
    answers = answers.readlines()
    answers = [a.rstrip() for a in answers]

result = list()

for word_data in dictionary:
    new_data = list()
    word, count = word_data.split()
    new_data.append(word)
    new_data.append(count)
    if word in answers:
        new_data.append("easy")
    else:
        new_data.append("hard")
    new_data = ' '.join(new_data)
    result.append(new_data)

with open("new_dictionary.txt", 'w') as new:
    new.write('\n'.join(result))
