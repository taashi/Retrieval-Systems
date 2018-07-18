import collections
import math
import re

catalogofStopWords = []
CORPUS = "/Users/rohitchawla/PycharmProjects/Phase 2 snippet generation/Corpus/"
STOPWORDS = "common_words.txt"

CACMREL = "cacm.rel.txt"

TEXTQUERIES = "FilteredQueries.txt"
qCount = {}
BM25SCORE = "bm25.txt"


def producingBrief():
    Scores = open(BM25SCORE, "r").readlines()
    for i in range(1,65):
        x = (i-1) * 100
        y = i * 100 -1
        newq = Scores[x:y]
        qNumber = str(i)
        qdTogether = {}
        for entry in newq[:10]:
            entry = re.split(r' ',entry)
            dIdentification = entry[2]
            qIdentification = entry[0]
            if not qdTogether.__contains__(qIdentification):
                qdTogether[qIdentification] = [dIdentification]
            else :
                qdTogether[qIdentification].append(dIdentification)
        producingSnippet(qNumber,qdTogether)


def producingSnippet(cacmqNumber,qdTogether):
    Snippets = open("Snippetsss/" +"Q" + cacmqNumber + ".txt", "w")
    for file in qdTogether[cacmqNumber]:
        if int(file) > 999 :
            file = "CACM-" + file + ".txt"
        elif int(file) > 99 and int(file) <=999:
            file = "CACM-0" + file + ".txt"
        elif int(file) <= 99 and int(file) >=10:
            file = "CACM-00" + file + ".txt"
        else:
            file = "CACM-000" + file + ".txt"
        openCorpus = open(CORPUS + file, "r")
        lines = openCorpus.read().split("\n")
        abstract = []
        inc = 1
        snip = {}
        g = 0
        while g < len(lines):
            snip[lines[g]] = sigValue(lines[g],qCount[int(cacmqNumber)])
            g = g + 1
        snipSorted = collections.OrderedDict(sorted(snip.items()))
        for v in snipSorted :
            if inc < 6:
                abstract.append(str(v))
                inc = inc + 1
        Snippets.write(openCorpus.name.split("\\")[-1].replace(".txt",""))
        Snippets.write("\n")
        q = 0
        while q< len(abstract):
            phrases = abstract[q].split()
            if len(phrases)!=0:
                t = 0
                while t< len(phrases):
                    if letterQSimilarity(qCount[int(cacmqNumber)],phrases[t]):
                        phrases[phrases.index(phrases[t])] = phrases[t].upper()
                        Snippets.write(phrases[t].upper())
                        Snippets.write(" ")
                    else :
                        Snippets.write(phrases[t] + " ")
                    t = t+ 1
                Snippets.write("\n")
            q = q+1
        Snippets.write("****************************")
        Snippets.write("\n")

def letterQSimilarity(inp,letter):
    removeSpcl = re.compile(r'[,.!\"();:<>-_+=@#]').sub('', letter)
    i = 0
    while i < len(inp.split("\n")):
        if inp.split("\n") != removeSpcl:
            return 0
    return 1

def sigValue(line,inp):
    line = line.split("\n")
    large = 0
    small = 0
    inc = 0
    j = 0
    while j< len(line[small:(large+1)]):
        if not catalogofStopWords.__contains__(line[small: (large+1)][j]) and letterQSimilarity(inp, line[small: (large+1)][j]):
            inc = inc +1
            break
        j = j + 1
    k = 0
    while k < len(range(len(line)-1,0,-1)):
        if not catalogofStopWords.__contains__(line[k]) and letterQSimilarity(inp,line[k]):
            large = k
            break
        k = k + 1
    m = 0
    while m < len(line):
        if not catalogofStopWords.__contains__(line[m]) and letterQSimilarity(inp, line[m]):
            small = line.index(line[m])
            break
        m = m+1
    if (len(line[small:(large+1)]) ==0):
        answer = 0
    else:
        answer = math.pow(inc,2)/ len(line[small:(large+1)])
    return answer



def begin():
    global catalogofStopWords,qCount
    readLines = open(STOPWORDS,"r").readlines()
    map(lambda x: catalogofStopWords.append(x.strip("\n")),readLines)
    readingQfile = open(TEXTQUERIES, "r").readlines()
    i = 0
    while i<len(readingQfile):
        qCount[i] = readingQfile[i]
        i = i+1
    producingBrief()


begin()
