from math import sqrt
from math import log
import glob, os


source = "C:\\Users\\Ravi\\Desktop\\New Folder (3)\\New folder\\Corpus\\"
result_location = "C:\\Users\\Ravi\\Desktop\\New Folder (3)\\New folder\\answers\\"
query_location = "C:\\Users\\Ravi\\Desktop\\New Folder (3)\\New folder\\FilteredQueries.txt"
relevance_location = 'C:\\Users\\Ravi\\Desktop\\New Folder (3)\\New folder\\cacm.rel.txt'
id_file_mapping = dict()
size_id_mapping = dict()
input_i = 0
mean_len = 0
resp = 1
os.chdir(source)
texts_in_folder = glob.glob("*.txt")
file_path = os.getcwd()

def calculation(input_q,inv_ind, file_count,important):
    new_inv_ind = dict()
    word_rate = dict()
    i = 0
    while i < len(input_q.split()):
        if word_rate.__contains__(input_q.split()[i]):
            word_rate[input_q.split()[i]] = word_rate[input_q.split()[i]] + 1
        else:
            word_rate[input_q.split()[i]] = 1
        i = i + 1
    for word in word_rate:
        if not inv_ind.__contains__(word):
            new_inv_ind[word] = dict()
        else:
            new_inv_ind[word] = inv_ind[word]
    val_calc(word_rate,new_inv_ind,file_count,important,input_q,inv_ind)

def val_calc(input_word,inv_ind,file_count,important,input_q,diction):
    file_val =dict()
    for word in inv_ind:
        rel_value = 0
        for file_uniq in inv_ind[word]:
            if file_uniq in important:
                rel_value = rel_value + 1
        for file_uniq in inv_ind[word]:
            one = log(((rel_value + 0.5) / (len(important) - rel_value + 0.5)) / ((len(inv_ind[word]) - rel_value + 0.5) / (file_count - len(inv_ind[word]) - len(important) + rel_value + 0.5)))
            two = ((1.2 + 1) * inv_ind[word][file_uniq]) / (1.2 * ((1 - 0.75) + 0.75 * (float(size_id_mapping[file_uniq]) / float(mean_len))) + inv_ind[word][file_uniq])
            three = ((100 + 1) * input_word[word]) / (100 + input_word[word])
            value =  one * two * three
            if not file_val.__contains__(file_uniq):
                file_val[file_uniq] = value
            else:
                value_sum = file_val[file_uniq] + value
                file_val[file_uniq] = value_sum
    file_val_desc = sorted(file_val.items(), key= lambda x: x[1],reverse=True)
    feedback(file_val_desc, input_q, diction, file_count, important)

def feedback(file_val_desc, input_q, diction, file_count, important):
    global resp
    if resp == 1:
        prf(file_val_desc, input_q, diction, file_count, important)
    else:
        resp = 1
        value_on_file(file_val_desc)

def queryvector(input_q,input_qv):
    e = 0
    while e < len(input_q.split()):
        if not input_qv.__contains__(input_q.split()[e]):
            input_qv[input_q.split()[e]] = 1
        else:
            input_qv[input_q.split()[e]] = input_qv[input_q.split()[e]] + 1
        e = e + 1

def prf(file_val_desc,input_q,inv_ind,file_count,important):
    global resp
    input_qv = dict()
    queryvector(input_q, input_qv)
    for word in inv_ind:
        if input_qv.__contains__(word):
            continue
        else:
            input_qv[word] = 0
    imp_diction = dict()
    a = 0
    while a < len(range(0,10)):
        file = open(source + "\\" + id_file_mapping[file_val_desc[range(0,10)[a]][0]] + ".txt").read()
        b = 0
        while b < len(file.split()):
            if not imp_diction.__contains__(file.split()[b]):
                imp_diction[file.split()[b]] = 1
            else:
                imp_diction[file.split()[b]] = imp_diction[file.split()[b]] + 1
            b = b + 1
        a = a + 1
    for word in inv_ind:
        if not imp_diction.__contains__(word):
            imp_diction[word] = 0
    imps = 0
    for word in imp_diction:
        imps = float(sqrt(float(imp_diction[word]**2) + imps))
    non_imp_diction = dict()
    q = 0
    while q < len(range(11,len(file_val_desc))):
        file = open(source + "\\" + id_file_mapping[file_val_desc[range(11,len(file_val_desc))[q]][0]] + ".txt").read()
        r = 0
        split_file = file.split()
        while r < len(split_file):
            if not non_imp_diction.__contains__(split_file[r]):
                non_imp_diction[split_file[r]] = 1
            else:
                non_imp_diction[split_file[r]] = non_imp_diction[split_file[r]] + 1
            r = r + 1
        q = q + 1
    for word in inv_ind:
        if not non_imp_diction.__contains__(word):
            non_imp_diction[word] = 0
    n_imps = 0
    for word in non_imp_diction:
        n_imps = n_imps + float(non_imp_diction[word]**2)
    changed_input = dict()
    for word in inv_ind:
        changed_input[word] = input_qv[word] + (0.5/imps) * imp_diction[word] - (0.15/float(sqrt(n_imps))) * non_imp_diction[word]
    sorted_changed_input = sorted(changed_input.items(), key=lambda x: x[1], reverse=True)
    input_copy = input_q
    c = 0
    while c < len(range(20)):
        word = sorted_changed_input[range(20)[c]][0]
        if not input_q.__contains__(word):
            input_copy = input_copy + " " + word
        c = c + 1
    resp = resp + 1
    calculation(input_copy,inv_ind,file_count,important)

def value_on_file(file_val_desc):
    openf  = open(result_location+"\\BM25_Model_Pseudo_Relevance.txt",'a')
    size = len(file_val_desc)
    if(size>= 100):
        for i in range(0,100):
            openf.write(str(input_i) + " Q0 " + \
            id_file_mapping[file_val_desc[i][0]][5:] \
            + " " + str(i + 1) + " " + str(file_val_desc[i][1]) \
            + " BM25_Model_Pseudo_Relevance\n")
    else:
        for i in range(0, len(file_val_desc)):
            openf.write(str(input_i) + " Q0 " + \
                            id_file_mapping[file_val_desc[i][0]][5:] \
                            + " " + str(i + 1) + " " + str(file_val_desc[i][1]) \
                            + " BM25_Model_Pseudo_Relevance\n")
    print("Processing "+ str(input_i))


def queryreader(inv_ind, file_count):
    global resp, input_i
    openf = open(query_location, 'r')
    for input_q in openf.readlines():
        input_i = input_i + 1
        resp = 1
        file_list = list()
        important = list()
        i = 0
        imp_open = open(relevance_location, 'r').readlines()
        while i < len(imp_open):
            each = imp_open[i].split(" ")
            if (str(input_i) == each[0]):
                file_list.insert(-1, each[2])
            i = i + 1
        for file_uniq in id_file_mapping:
            if id_file_mapping.get(file_uniq) in file_list:
                important.insert(-1, file_uniq)
        calculation(input_q, inv_ind, file_count, important)


def begin():
    global mean_len
    inv_ind = dict()
    file_uniq = 1
    for file in texts_in_folder:
        id_file_mapping[file_uniq] = file[:-4]
        open_f = open(file, 'r').read()
        material = open_f.split()
        size_id_mapping[file_uniq] = len(open_f.split())
        for word in material:
            if not inv_ind.__contains__(word):
                inv_ind.update({word: {file_uniq: 1}})
            elif file_uniq in inv_ind[word]:
                inv_ind[word][file_uniq] = inv_ind[word][file_uniq] + 1
            else:
                inv_ind[word].update({file_uniq: 1})
        file_uniq = file_uniq + 1
    file_count = file_uniq - 1
    open(result_location + "\\BM25_Model_Pseudo_Relevance.txt", 'w').close()
    mean_len = (float(sum(size_id_mapping.values())) / float(len(size_id_mapping)))
    queryreader(inv_ind, file_count)

begin()