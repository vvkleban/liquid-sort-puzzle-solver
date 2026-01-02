A memory optimized A* and BFS based solver for liquid sort puzzles. Both algorithms will guarantee the solution (if any) will contain the fewest possible moves. A* has a heuristic making it much faster and use less memory than BFS. The BFS is memory and performance optimized over classic Dijkstra's algorithm, storing moves in vectors, which are pruned along with collision hashes

```
liquid_sort_solver --help
Uses an algorithm based on command line arguments to sort liquids in bottles

Usage: liquid_sort_solver [OPTIONS] <puzzle data

Options:
      --bfs      Use the BFS algorithm
      --astar    Use the A* algorithm (default)
  -h, --help     Print help
  -V, --version  Print version
```

Where input.txt is structured in the following fashion:
Each non-comment line is a puzzle row, where bottles are semicolon or newline separated. Each bottle can have up to 4 characters each representing a color. If less than 4 colors per bottle are specified, the rest are assumed to be empty spaces
Newline characters between the bottles shape the output rows for each step. Lines starting with `#` are ignored

Sample file contents (mind 3 empty bottles at the end):
```
PSED;BPED;GUBP;OHLL;COAL;YYOA;YCUR;YORS;GNNA;CUUS;PNDA;GCEN;GHHD;BBSE;RRHL;;;
#Green, Orange, pUrple, Red, grAy, Lightgreen, Yellow, Blue, Swamp, browN, Cyan, Pink, bEige, cHartreuse, Darkblue
```

This is what the shortest possible solution of this puzzle will look like:
```
liquid_sort_solver <input1100_extra_bottle.txt
Step 0
|D||D||P||L||L||A|  |R||S||A||S||A||N|  |D||E||L|| || || |
|E||E||B||L||A||O|  |U||R||N||U||D||E|  |H||S||H|| || || |
|S||P||U||H||O||Y|  |C||O||N||U||N||C|  |H||B||R|| || || |
|P||B||G||O||C||Y|  |Y||Y||G||C||P||G|  |G||B||R|| || || |
------------------------------------------------

Step 1
❚ ❚|D||P||L||L||A|  |R||S||A||S||A||N|  |D||E||L|❚ ❚| || |
❚E❚|E||B||L||A||O|  |U||R||N||U||D||E|  |H||S||H|❚ ❚| || |
❚S❚|P||U||H||O||Y|  |C||O||N||U||N||C|  |H||B||R|❚ ❚| || |
❚P❚|B||G||O||C||Y|  |Y||Y||G||C||P||G|  |G||B||R|❚D❚| || |
------------------------------------------------

Step 2
| ||D||P||L||L||A|  |R||S||A||S||A||N|  |D||E|❚ ❚| |❚ ❚| |
|E||E||B||L||A||O|  |U||R||N||U||D||E|  |H||S|❚H❚| |❚ ❚| |
|S||P||U||H||O||Y|  |C||O||N||U||N||C|  |H||B|❚R❚| |❚ ❚| |
|P||B||G||O||C||Y|  |Y||Y||G||C||P||G|  |G||B|❚R❚|D|❚L❚| |
------------------------------------------------

Step 3
| ||D||P||L||L|❚ ❚  |R||S||A||S||A||N|  |D||E|| || || |❚ ❚
|E||E||B||L||A|❚O❚  |U||R||N||U||D||E|  |H||S||H|| || |❚ ❚
|S||P||U||H||O|❚Y❚  |C||O||N||U||N||C|  |H||B||R|| || |❚ ❚
|P||B||G||O||C|❚Y❚  |Y||Y||G||C||P||G|  |G||B||R||D||L|❚A❚
------------------------------------------------

Step 4
| ||D||P||L|❚ ❚| |  |R||S||A||S||A||N|  |D||E|| || |❚ ❚| |
|E||E||B||L|❚A❚|O|  |U||R||N||U||D||E|  |H||S||H|| |❚ ❚| |
|S||P||U||H|❚O❚|Y|  |C||O||N||U||N||C|  |H||B||R|| |❚L❚| |
|P||B||G||O|❚C❚|Y|  |Y||Y||G||C||P||G|  |G||B||R||D|❚L❚|A|
------------------------------------------------

Step 5
| ||D||P||L|❚ ❚| |  |R||S||A||S||A||N|  |D||E|| || || |❚ ❚
|E||E||B||L|❚ ❚|O|  |U||R||N||U||D||E|  |H||S||H|| || |❚ ❚
|S||P||U||H|❚O❚|Y|  |C||O||N||U||N||C|  |H||B||R|| ||L|❚A❚
|P||B||G||O|❚C❚|Y|  |Y||Y||G||C||P||G|  |G||B||R||D||L|❚A❚
------------------------------------------------

Step 6
| ||D||P|❚ ❚| || |  |R||S||A||S||A||N|  |D||E|| || |❚L❚| |
|E||E||B|❚ ❚| ||O|  |U||R||N||U||D||E|  |H||S||H|| |❚L❚| |
|S||P||U|❚H❚|O||Y|  |C||O||N||U||N||C|  |H||B||R|| |❚L❚|A|
|P||B||G|❚O❚|C||Y|  |Y||Y||G||C||P||G|  |G||B||R||D|❚L❚|A|
------------------------------------------------

Step 7
| ||D||P|| || || |  |R||S||A||S||A||N|  ❚ ❚|E|| |❚ ❚|L|| |
|E||E||B|| || ||O|  |U||R||N||U||D||E|  ❚H❚|S||H|❚ ❚|L|| |
|S||P||U||H||O||Y|  |C||O||N||U||N||C|  ❚H❚|B||R|❚D❚|L||A|
|P||B||G||O||C||Y|  |Y||Y||G||C||P||G|  ❚G❚|B||R|❚D❚|L||A|
------------------------------------------------

Step 8
| ||D||P|❚ ❚| || |  |R||S||A||S||A||N|  | ||E|❚ ❚| ||L|| |
|E||E||B|❚H❚| ||O|  |U||R||N||U||D||E|  |H||S|❚ ❚| ||L|| |
|S||P||U|❚H❚|O||Y|  |C||O||N||U||N||C|  |H||B|❚R❚|D||L||A|
|P||B||G|❚O❚|C||Y|  |Y||Y||G||C||P||G|  |G||B|❚R❚|D||L||A|
------------------------------------------------

Step 9
| ||D||P|| || || |  ❚ ❚|S||A||S||A||N|  | ||E|❚ ❚| ||L|| |
|E||E||B||H|| ||O|  ❚U❚|R||N||U||D||E|  |H||S|❚R❚| ||L|| |
|S||P||U||H||O||Y|  ❚C❚|O||N||U||N||C|  |H||B|❚R❚|D||L||A|
|P||B||G||O||C||Y|  ❚Y❚|Y||G||C||P||G|  |G||B|❚R❚|D||L||A|
------------------------------------------------

Step 10
❚E❚|D||P|| || || |  | ||S||A||S||A||N|  | |❚ ❚| || ||L|| |
❚E❚|E||B||H|| ||O|  |U||R||N||U||D||E|  |H|❚S❚|R|| ||L|| |
❚S❚|P||U||H||O||Y|  |C||O||N||U||N||C|  |H|❚B❚|R||D||L||A|
❚P❚|B||G||O||C||Y|  |Y||Y||G||C||P||G|  |G|❚B❚|R||D||L||A|
------------------------------------------------

Step 11
|E||D||P|| || || |  | |❚ ❚|A||S||A||N|  | |❚S❚| || ||L|| |
|E||E||B||H|| ||O|  |U|❚R❚|N||U||D||E|  |H|❚S❚|R|| ||L|| |
|S||P||U||H||O||Y|  |C|❚O❚|N||U||N||C|  |H|❚B❚|R||D||L||A|
|P||B||G||O||C||Y|  |Y|❚Y❚|G||C||P||G|  |G|❚B❚|R||D||L||A|
------------------------------------------------

Step 12
|E|❚ ❚|P|| || || |  | || ||A||S||A||N|  | ||S|| |❚ ❚|L|| |
|E|❚E❚|B||H|| ||O|  |U||R||N||U||D||E|  |H||S||R|❚D❚|L|| |
|S|❚P❚|U||H||O||Y|  |C||O||N||U||N||C|  |H||B||R|❚D❚|L||A|
|P|❚B❚|G||O||C||Y|  |Y||Y||G||C||P||G|  |G||B||R|❚D❚|L||A|
------------------------------------------------

Step 13
|E|| ||P|| || || |  | || ||A||S|❚ ❚|N|  | ||S|| || ||L|❚ ❚
|E||E||B||H|| ||O|  |U||R||N||U|❚D❚|E|  |H||S||R||D||L|❚A❚
|S||P||U||H||O||Y|  |C||O||N||U|❚N❚|C|  |H||B||R||D||L|❚A❚
|P||B||G||O||C||Y|  |Y||Y||G||C|❚P❚|G|  |G||B||R||D||L|❚A❚
------------------------------------------------

Step 14
|E|| ||P|| || || |  | || ||A||S|❚ ❚|N|  | ||S|| |❚D❚|L|| |
|E||E||B||H|| ||O|  |U||R||N||U|❚ ❚|E|  |H||S||R|❚D❚|L||A|
|S||P||U||H||O||Y|  |C||O||N||U|❚N❚|C|  |H||B||R|❚D❚|L||A|
|P||B||G||O||C||Y|  |Y||Y||G||C|❚P❚|G|  |G||B||R|❚D❚|L||A|
------------------------------------------------

Step 15
|E|| ||P|| || || |  | || ||A||S|❚ ❚❚ ❚  | ||S|| ||D||L|| |
|E||E||B||H|| ||O|  |U||R||N||U|❚N❚❚E❚  |H||S||R||D||L||A|
|S||P||U||H||O||Y|  |C||O||N||U|❚N❚❚C❚  |H||B||R||D||L||A|
|P||B||G||O||C||Y|  |Y||Y||G||C|❚P❚❚G❚  |G||B||R||D||L||A|
------------------------------------------------

Step 16
|E|❚ ❚|P|| || || |  | || ||A||S|| |❚E❚  | ||S|| ||D||L|| |
|E|❚ ❚|B||H|| ||O|  |U||R||N||U||N|❚E❚  |H||S||R||D||L||A|
|S|❚P❚|U||H||O||Y|  |C||O||N||U||N|❚C❚  |H||B||R||D||L||A|
|P|❚B❚|G||O||C||Y|  |Y||Y||G||C||P|❚G❚  |G||B||R||D||L||A|
------------------------------------------------

Step 17
|E|| ||P|| || || |  | |❚ ❚|A||S|| ||E|  | ||S|❚R❚|D||L|| |
|E|| ||B||H|| ||O|  |U|❚ ❚|N||U||N||E|  |H||S|❚R❚|D||L||A|
|S||P||U||H||O||Y|  |C|❚O❚|N||U||N||C|  |H||B|❚R❚|D||L||A|
|P||B||G||O||C||Y|  |Y|❚Y❚|G||C||P||G|  |G||B|❚R❚|D||L||A|
------------------------------------------------

Step 18
|E|❚ ❚❚ ❚| || || |  | || ||A||S|| ||E|  | ||S||R||D||L|| |
|E|❚P❚❚B❚|H|| ||O|  |U|| ||N||U||N||E|  |H||S||R||D||L||A|
|S|❚P❚❚U❚|H||O||Y|  |C||O||N||U||N||C|  |H||B||R||D||L||A|
|P|❚B❚❚G❚|O||C||Y|  |Y||Y||G||C||P||G|  |G||B||R||D||L||A|
------------------------------------------------

Step 19
|E|| || || || |❚ ❚  | |❚ ❚|A||S|| ||E|  | ||S||R||D||L|| |
|E||P||B||H|| |❚ ❚  |U|❚O❚|N||U||N||E|  |H||S||R||D||L||A|
|S||P||U||H||O|❚Y❚  |C|❚O❚|N||U||N||C|  |H||B||R||D||L||A|
|P||B||G||O||C|❚Y❚  |Y|❚Y❚|G||C||P||G|  |G||B||R||D||L||A|
------------------------------------------------

Step 20
|E|| || || |❚O❚| |  | |❚ ❚|A||S|| ||E|  | ||S||R||D||L|| |
|E||P||B||H|❚O❚| |  |U|❚ ❚|N||U||N||E|  |H||S||R||D||L||A|
|S||P||U||H|❚O❚|Y|  |C|❚ ❚|N||U||N||C|  |H||B||R||D||L||A|
|P||B||G||O|❚C❚|Y|  |Y|❚Y❚|G||C||P||G|  |G||B||R||D||L||A|
------------------------------------------------

Step 21
|E|| || || ||O|❚ ❚  | |❚ ❚|A||S|| ||E|  | ||S||R||D||L|| |
|E||P||B||H||O|❚Y❚  |U|❚ ❚|N||U||N||E|  |H||S||R||D||L||A|
|S||P||U||H||O|❚Y❚  |C|❚ ❚|N||U||N||C|  |H||B||R||D||L||A|
|P||B||G||O||C|❚Y❚  |Y|❚ ❚|G||C||P||G|  |G||B||R||D||L||A|
------------------------------------------------

Step 22
|E|| || || ||O|| |  | |❚ ❚|A||S|❚ ❚|E|  | ||S||R||D||L|| |
|E||P||B||H||O||Y|  |U|❚ ❚|N||U|❚ ❚|E|  |H||S||R||D||L||A|
|S||P||U||H||O||Y|  |C|❚N❚|N||U|❚ ❚|C|  |H||B||R||D||L||A|
|P||B||G||O||C||Y|  |Y|❚N❚|G||C|❚P❚|G|  |G||B||R||D||L||A|
------------------------------------------------

Step 23
|E|❚ ❚| || ||O|| |  | || ||A||S|❚ ❚|E|  | ||S||R||D||L|| |
|E|❚ ❚|B||H||O||Y|  |U|| ||N||U|❚P❚|E|  |H||S||R||D||L||A|
|S|❚ ❚|U||H||O||Y|  |C||N||N||U|❚P❚|C|  |H||B||R||D||L||A|
|P|❚B❚|G||O||C||Y|  |Y||N||G||C|❚P❚|G|  |G||B||R||D||L||A|
------------------------------------------------

Step 24
|E|| || || ||O|| |  | || |❚ ❚|S|| ||E|  | ||S||R||D||L|❚A❚
|E|| ||B||H||O||Y|  |U|| |❚N❚|U||P||E|  |H||S||R||D||L|❚A❚
|S|| ||U||H||O||Y|  |C||N|❚N❚|U||P||C|  |H||B||R||D||L|❚A❚
|P||B||G||O||C||Y|  |Y||N|❚G❚|C||P||G|  |G||B||R||D||L|❚A❚
------------------------------------------------

Step 25
|E|❚ ❚❚B❚| ||O|| |  | || || ||S|| ||E|  | ||S||R||D||L||A|
|E|❚ ❚❚B❚|H||O||Y|  |U|| ||N||U||P||E|  |H||S||R||D||L||A|
|S|❚ ❚❚U❚|H||O||Y|  |C||N||N||U||P||C|  |H||B||R||D||L||A|
|P|❚ ❚❚G❚|O||C||Y|  |Y||N||G||C||P||G|  |G||B||R||D||L||A|
------------------------------------------------

Step 26
❚ ❚❚ ❚|B|| ||O|| |  | || || ||S|| ||E|  | ||S||R||D||L||A|
❚ ❚❚ ❚|B||H||O||Y|  |U|| ||N||U||P||E|  |H||S||R||D||L||A|
❚S❚❚E❚|U||H||O||Y|  |C||N||N||U||P||C|  |H||B||R||D||L||A|
❚P❚❚E❚|G||O||C||Y|  |Y||N||G||C||P||G|  |G||B||R||D||L||A|
------------------------------------------------

Step 27
| |❚E❚|B|| ||O|| |  | || || ||S|| |❚ ❚  | ||S||R||D||L||A|
| |❚E❚|B||H||O||Y|  |U|| ||N||U||P|❚ ❚  |H||S||R||D||L||A|
|S|❚E❚|U||H||O||Y|  |C||N||N||U||P|❚C❚  |H||B||R||D||L||A|
|P|❚E❚|G||O||C||Y|  |Y||N||G||C||P|❚G❚  |G||B||R||D||L||A|
------------------------------------------------

Step 28
| ||E||B|| ||O|| |  | |❚N❚❚ ❚|S|| || |  | ||S||R||D||L||A|
| ||E||B||H||O||Y|  |U|❚N❚❚ ❚|U||P|| |  |H||S||R||D||L||A|
|S||E||U||H||O||Y|  |C|❚N❚❚ ❚|U||P||C|  |H||B||R||D||L||A|
|P||E||G||O||C||Y|  |Y|❚N❚❚G❚|C||P||G|  |G||B||R||D||L||A|
------------------------------------------------

Step 29
❚S❚|E||B|| ||O|| |  | ||N|| ||S|| || |  | |❚ ❚|R||D||L||A|
❚S❚|E||B||H||O||Y|  |U||N|| ||U||P|| |  |H|❚ ❚|R||D||L||A|
❚S❚|E||U||H||O||Y|  |C||N|| ||U||P||C|  |H|❚B❚|R||D||L||A|
❚P❚|E||G||O||C||Y|  |Y||N||G||C||P||G|  |G|❚B❚|R||D||L||A|
------------------------------------------------

Step 30
|S||E|❚ ❚| ||O|| |  | ||N|| ||S|| || |  | |❚B❚|R||D||L||A|
|S||E|❚ ❚|H||O||Y|  |U||N|| ||U||P|| |  |H|❚B❚|R||D||L||A|
|S||E|❚U❚|H||O||Y|  |C||N|| ||U||P||C|  |H|❚B❚|R||D||L||A|
|P||E|❚G❚|O||C||Y|  |Y||N||G||C||P||G|  |G|❚B❚|R||D||L||A|
------------------------------------------------

Step 31
|S||E|❚ ❚| ||O|| |  ❚ ❚|N|| ||S|| || |  | ||B||R||D||L||A|
|S||E|❚U❚|H||O||Y|  ❚ ❚|N|| ||U||P|| |  |H||B||R||D||L||A|
|S||E|❚U❚|H||O||Y|  ❚C❚|N|| ||U||P||C|  |H||B||R||D||L||A|
|P||E|❚G❚|O||C||Y|  ❚Y❚|N||G||C||P||G|  |G||B||R||D||L||A|
------------------------------------------------

Step 32
|S||E|| || ||O|| |  ❚ ❚|N|| ||S|| |❚ ❚  | ||B||R||D||L||A|
|S||E||U||H||O||Y|  ❚C❚|N|| ||U||P|❚ ❚  |H||B||R||D||L||A|
|S||E||U||H||O||Y|  ❚C❚|N|| ||U||P|❚ ❚  |H||B||R||D||L||A|
|P||E||G||O||C||Y|  ❚Y❚|N||G||C||P|❚G❚  |G||B||R||D||L||A|
------------------------------------------------

Step 33
|S||E|| || ||O|| |  | ||N|❚ ❚|S|| |❚ ❚  | ||B||R||D||L||A|
|S||E||U||H||O||Y|  |C||N|❚ ❚|U||P|❚ ❚  |H||B||R||D||L||A|
|S||E||U||H||O||Y|  |C||N|❚G❚|U||P|❚ ❚  |H||B||R||D||L||A|
|P||E||G||O||C||Y|  |Y||N|❚G❚|C||P|❚ ❚  |G||B||R||D||L||A|
------------------------------------------------

Step 34
|S||E|| || ||O|| |  | ||N|| ||S|| |❚ ❚  ❚ ❚|B||R||D||L||A|
|S||E||U||H||O||Y|  |C||N|| ||U||P|❚ ❚  ❚ ❚|B||R||D||L||A|
|S||E||U||H||O||Y|  |C||N||G||U||P|❚H❚  ❚ ❚|B||R||D||L||A|
|P||E||G||O||C||Y|  |Y||N||G||C||P|❚H❚  ❚G❚|B||R||D||L||A|
------------------------------------------------

Step 35
|S||E|| || ||O|| |  | ||N|❚ ❚|S|| || |  ❚ ❚|B||R||D||L||A|
|S||E||U||H||O||Y|  |C||N|❚G❚|U||P|| |  ❚ ❚|B||R||D||L||A|
|S||E||U||H||O||Y|  |C||N|❚G❚|U||P||H|  ❚ ❚|B||R||D||L||A|
|P||E||G||O||C||Y|  |Y||N|❚G❚|C||P||H|  ❚ ❚|B||R||D||L||A|
------------------------------------------------

Step 36
|S||E|| || ||O|| |  | ||N|| |❚ ❚| || |  ❚ ❚|B||R||D||L||A|
|S||E||U||H||O||Y|  |C||N||G|❚U❚|P|| |  ❚ ❚|B||R||D||L||A|
|S||E||U||H||O||Y|  |C||N||G|❚U❚|P||H|  ❚ ❚|B||R||D||L||A|
|P||E||G||O||C||Y|  |Y||N||G|❚C❚|P||H|  ❚S❚|B||R||D||L||A|
------------------------------------------------

Step 37
|S||E|| |❚ ❚|O|| |  | ||N|| || || |❚H❚  | ||B||R||D||L||A|
|S||E||U|❚ ❚|O||Y|  |C||N||G||U||P|❚H❚  | ||B||R||D||L||A|
|S||E||U|❚ ❚|O||Y|  |C||N||G||U||P|❚H❚  | ||B||R||D||L||A|
|P||E||G|❚O❚|C||Y|  |Y||N||G||C||P|❚H❚  |S||B||R||D||L||A|
------------------------------------------------

Step 38
❚ ❚|E|| || ||O|| |  | ||N|| || || ||H|  ❚S❚|B||R||D||L||A|
❚ ❚|E||U|| ||O||Y|  |C||N||G||U||P||H|  ❚S❚|B||R||D||L||A|
❚ ❚|E||U|| ||O||Y|  |C||N||G||U||P||H|  ❚S❚|B||R||D||L||A|
❚P❚|E||G||O||C||Y|  |Y||N||G||C||P||H|  ❚S❚|B||R||D||L||A|
------------------------------------------------

Step 39
❚P❚|E|| || ||O|| |  | ||N|| || |❚ ❚|H|  |S||B||R||D||L||A|
❚P❚|E||U|| ||O||Y|  |C||N||G||U|❚ ❚|H|  |S||B||R||D||L||A|
❚P❚|E||U|| ||O||Y|  |C||N||G||U|❚ ❚|H|  |S||B||R||D||L||A|
❚P❚|E||G||O||C||Y|  |Y||N||G||C|❚ ❚|H|  |S||B||R||D||L||A|
------------------------------------------------

Step 40
|P||E|| || ||O|| |  | ||N|| |❚ ❚❚ ❚|H|  |S||B||R||D||L||A|
|P||E||U|| ||O||Y|  |C||N||G|❚ ❚❚ ❚|H|  |S||B||R||D||L||A|
|P||E||U|| ||O||Y|  |C||N||G|❚ ❚❚U❚|H|  |S||B||R||D||L||A|
|P||E||G||O||C||Y|  |Y||N||G|❚C❚❚U❚|H|  |S||B||R||D||L||A|
------------------------------------------------

Step 41
|P||E|| |❚O❚❚ ❚| |  | ||N|| || || ||H|  |S||B||R||D||L||A|
|P||E||U|❚O❚❚ ❚|Y|  |C||N||G|| || ||H|  |S||B||R||D||L||A|
|P||E||U|❚O❚❚ ❚|Y|  |C||N||G|| ||U||H|  |S||B||R||D||L||A|
|P||E||G|❚O❚❚C❚|Y|  |Y||N||G||C||U||H|  |S||B||R||D||L||A|
------------------------------------------------

Step 42
|P||E|| ||O|❚ ❚| |  ❚C❚|N|| || || ||H|  |S||B||R||D||L||A|
|P||E||U||O|❚ ❚|Y|  ❚C❚|N||G|| || ||H|  |S||B||R||D||L||A|
|P||E||U||O|❚ ❚|Y|  ❚C❚|N||G|| ||U||H|  |S||B||R||D||L||A|
|P||E||G||O|❚ ❚|Y|  ❚Y❚|N||G||C||U||H|  |S||B||R||D||L||A|
------------------------------------------------

Step 43
|P||E|| ||O|| || |  ❚ ❚|N|| |❚C❚| ||H|  |S||B||R||D||L||A|
|P||E||U||O|| ||Y|  ❚ ❚|N||G|❚C❚| ||H|  |S||B||R||D||L||A|
|P||E||U||O|| ||Y|  ❚ ❚|N||G|❚C❚|U||H|  |S||B||R||D||L||A|
|P||E||G||O|| ||Y|  ❚Y❚|N||G|❚C❚|U||H|  |S||B||R||D||L||A|
------------------------------------------------

Step 44
|P||E|| ||O|| |❚Y❚  ❚ ❚|N|| ||C|| ||H|  |S||B||R||D||L||A|
|P||E||U||O|| |❚Y❚  ❚ ❚|N||G||C|| ||H|  |S||B||R||D||L||A|
|P||E||U||O|| |❚Y❚  ❚ ❚|N||G||C||U||H|  |S||B||R||D||L||A|
|P||E||G||O|| |❚Y❚  ❚ ❚|N||G||C||U||H|  |S||B||R||D||L||A|
------------------------------------------------

Step 45
|P||E|❚ ❚|O|| ||Y|  | ||N|| ||C|❚U❚|H|  |S||B||R||D||L||A|
|P||E|❚ ❚|O|| ||Y|  | ||N||G||C|❚U❚|H|  |S||B||R||D||L||A|
|P||E|❚ ❚|O|| ||Y|  | ||N||G||C|❚U❚|H|  |S||B||R||D||L||A|
|P||E|❚G❚|O|| ||Y|  | ||N||G||C|❚U❚|H|  |S||B||R||D||L||A|
------------------------------------------------

Step 46
|P||E|❚G❚|O|| ||Y|  | ||N|❚ ❚|C||U||H|  |S||B||R||D||L||A|
|P||E|❚G❚|O|| ||Y|  | ||N|❚ ❚|C||U||H|  |S||B||R||D||L||A|
|P||E|❚G❚|O|| ||Y|  | ||N|❚ ❚|C||U||H|  |S||B||R||D||L||A|
|P||E|❚G❚|O|| ||Y|  | ||N|❚ ❚|C||U||H|  |S||B||R||D||L||A|
------------------------------------------------

```
