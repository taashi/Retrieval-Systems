# CS6200-project

`./provided` - Contains all provided documents necessary for running the pieces of the project

`./parsed`  - Contains the documents after html stripping, punctuation removal, and case folding

`./indexer` - Contains code for parsing and indexing both the stemmed and unstemmed corpus documents

`./retriever` - Contains code for BM25, tfidf, Smoothed Query Likelihood, and Lucene

`./evaluation` - Contains code for all retrieval metrics

`./snippet-generation` - Contains code for snippet generation

index, retriever, and evaluation directories each cantains a README describing how to compile/run that part

`./final-report.pdf` - Final report containing implementation details, results, and conclusions

Lucene - i) Create a new java project and add Lucene.java to it.
    ii) add the following three jars into your project's list of referenced libraries:-
        a) lucene-core-4.7.2.jar
        b) lucene-queryparser-4.7.2.jar
        c) lucene-analyzers-common-4.7.2.jar
   iii) In the code replace data variable with the path of your cacm query file.
    iii) Run the file
    iv) When prompted 'Enter the FULL path where the index will be created: (e.g. /Usr/index or c:\temp\index)'
        Enter the path where you want your index to be created
    v) When prompted 'Enter the FULL path to add into the index (q=quit): (e.g. /home/mydir/docs or c:\Users\mydir\docs)
                [Acceptable file types: .xml, .html, .html, .txt]'
        Enter the path of your Corpus
    vi) Now the above statement is prompted again, enter 'q' this time
    vii) All the Query ranked files will be generated at the location of your java project
    viii) The text in files is in the following format:
<query number> Q0 <numerical doc id> <rank> <score> Lucene

Snippet Generation - Place the code in the same folder as the corpus, cacm.rel.txt, common_words.txt, FilteredQueries and bm25.txt. Mention the path or place all the files mentioned in the same folder as the code. Create another folder called Snipetsss where for all the queries the snippets would be generated. Run the code.
This code was developed using Pycharm and Python 3.

Bm 25 Psuedo Relevance Feedback : -
i) Create a new python project and add Task2.py to it.
ii) Update the 'result_location' variable. Enter the location where you want to keep your BM Score ranking files.
iii) Change 'source' value. Put the path of the Corpus folder.
iv) Change the value of 'query_location' Put the path of 'FilteredQueries.txt'
v) Change the value of 'relevance_location' Put the path of 'cacm.rel.txt'
v) Run the file. The files would be generated at the result location for every query in the queries in file.
vi) The text in file is in the following format: query_id Q0 doc_id rank BM25_score system_name
