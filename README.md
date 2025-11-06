# Mireiniwr
A cyber security tool to identify sensitive files on a Windows NT system. The tool aims to use four main methods to find files:

1) Wide searches based on [filename extensions](https://gist.github.com/lgg/e6ccc6e212d18dd2ecd8a8c116fb1e45).

2) Wide searches using [file signatures](https://en.wikipedia.org/wiki/List_of_file_signatures) also known as [magic numbers](https://www.garykessler.net/library/file_sigs_GCK_latest.html).
   
3) Known locations of sensitive files for [common applications](https://easytechsolver.com/where-is-the-password-stored-in-chrome/).

4) Searching text files for very [high entropy strings](https://kee1ongz.github.io/paper/sp25-secret.pdf).

5) Searching for secrets in text files based on the presence of [certain strings or characters](https://aoa0.github.io/pubs/icse22.pdf). 
