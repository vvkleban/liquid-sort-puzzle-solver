A memory optimized BFS based solver for liquid sort puzzles. BFS will guarantee the solution (if any) will contain the fewest possible moves

Usage:
liquid_sort_solver <input.txt

Where input.txt is structiured in the following fashion:
- Each line should exactly contain four ASCII characters representing the content of the bottle. A ' ' (space) represents an empty space, while other characters indicate liquid colors
- There should be exactly 4 of each color (a character) in the file. Only space character is free from this limitation
- A line starting with '#' will be ignored
- An empty line will be ignored

Sample file contents:
```
# Beginning of file
ABCB
DEFE
GGHA
HFCD

IGAC
FHIE
EAHG
CIID

BBFD
    
    
# End of file
```
This is what the shortest possible solution of this puzzle will look like:
```
|B||E||A||D|  |C||E||G||D|  |D|| || |
|C||F||H||C|  |A||I||H||I|  |F|| || |
|B||E||G||F|  |G||H||A||I|  |B|| || |
|A||D||G||H|  |I||F||E||C|  |B|| || |
------------------------------------------------

|B|❚ ❚|A||D|  |C||E||G||D|  |D|❚ ❚| |
|C|❚F❚|H||C|  |A||I||H||I|  |F|❚ ❚| |
|B|❚E❚|G||F|  |G||H||A||I|  |B|❚ ❚| |
|A|❚D❚|G||H|  |I||F||E||C|  |B|❚E❚| |
------------------------------------------------

|B|| ||A||D|  |C|❚ ❚|G||D|  |D|❚ ❚| |
|C||F||H||C|  |A|❚I❚|H||I|  |F|❚ ❚| |
|B||E||G||F|  |G|❚H❚|A||I|  |B|❚E❚| |
|A||D||G||H|  |I|❚F❚|E||C|  |B|❚E❚| |
------------------------------------------------

|B|❚ ❚|A||D|  |C|| ||G||D|  |D|| |❚ ❚
|C|❚ ❚|H||C|  |A||I||H||I|  |F|| |❚ ❚
|B|❚E❚|G||F|  |G||H||A||I|  |B||E|❚ ❚
|A|❚D❚|G||H|  |I||F||E||C|  |B||E|❚F❚
------------------------------------------------

|B|❚ ❚|A||D|  |C|| ||G||D|  |D|❚ ❚| |
|C|❚ ❚|H||C|  |A||I||H||I|  |F|❚E❚| |
|B|❚ ❚|G||F|  |G||H||A||I|  |B|❚E❚| |
|A|❚D❚|G||H|  |I||F||E||C|  |B|❚E❚|F|
------------------------------------------------

|B|❚ ❚|A|❚ ❚  |C|| ||G||D|  |D|| || |
|C|❚ ❚|H|❚C❚  |A||I||H||I|  |F||E|| |
|B|❚D❚|G|❚F❚  |G||H||A||I|  |B||E|| |
|A|❚D❚|G|❚H❚  |I||F||E||C|  |B||E||F|
------------------------------------------------

|B|❚ ❚|A|| |  |C|| ||G|❚ ❚  |D|| || |
|C|❚D❚|H||C|  |A||I||H|❚I❚  |F||E|| |
|B|❚D❚|G||F|  |G||H||A|❚I❚  |B||E|| |
|A|❚D❚|G||H|  |I||F||E|❚C❚  |B||E||F|
------------------------------------------------

|B|❚D❚|A|| |  |C|| ||G|| |  ❚ ❚| || |
|C|❚D❚|H||C|  |A||I||H||I|  ❚F❚|E|| |
|B|❚D❚|G||F|  |G||H||A||I|  ❚B❚|E|| |
|A|❚D❚|G||H|  |I||F||E||C|  ❚B❚|E||F|
------------------------------------------------

|B||D||A|❚C❚  ❚ ❚| ||G|| |  | || || |
|C||D||H|❚C❚  ❚A❚|I||H||I|  |F||E|| |
|B||D||G|❚F❚  ❚G❚|H||A||I|  |B||E|| |
|A||D||G|❚H❚  ❚I❚|F||E||C|  |B||E||F|
------------------------------------------------

|B||D|❚ ❚|C|  ❚A❚| ||G|| |  | || || |
|C||D|❚H❚|C|  ❚A❚|I||H||I|  |F||E|| |
|B||D|❚G❚|F|  ❚G❚|H||A||I|  |B||E|| |
|A||D|❚G❚|H|  ❚I❚|F||E||C|  |B||E||F|
------------------------------------------------

|B||D|| ||C|  |A|❚ ❚|G|❚I❚  | || || |
|C||D||H||C|  |A|❚ ❚|H|❚I❚  |F||E|| |
|B||D||G||F|  |G|❚H❚|A|❚I❚  |B||E|| |
|A||D||G||H|  |I|❚F❚|E|❚C❚  |B||E||F|
------------------------------------------------

|B||D|❚ ❚|C|  |A|❚ ❚|G||I|  | || || |
|C||D|❚ ❚|C|  |A|❚H❚|H||I|  |F||E|| |
|B||D|❚G❚|F|  |G|❚H❚|A||I|  |B||E|| |
|A||D|❚G❚|H|  |I|❚F❚|E||C|  |B||E||F|
------------------------------------------------

|B||D|❚ ❚|C|  |A|| |❚ ❚|I|  | || || |
|C||D|❚G❚|C|  |A||H|❚H❚|I|  |F||E|| |
|B||D|❚G❚|F|  |G||H|❚A❚|I|  |B||E|| |
|A||D|❚G❚|H|  |I||F|❚E❚|C|  |B||E||F|
------------------------------------------------

|B||D|| ||C|  |A|❚H❚❚ ❚|I|  | || || |
|C||D||G||C|  |A|❚H❚❚ ❚|I|  |F||E|| |
|B||D||G||F|  |G|❚H❚❚A❚|I|  |B||E|| |
|A||D||G||H|  |I|❚F❚❚E❚|C|  |B||E||F|
------------------------------------------------

|B||D|| ||C|  ❚ ❚|H|❚A❚|I|  | || || |
|C||D||G||C|  ❚ ❚|H|❚A❚|I|  |F||E|| |
|B||D||G||F|  ❚G❚|H|❚A❚|I|  |B||E|| |
|A||D||G||H|  ❚I❚|F|❚E❚|C|  |B||E||F|
------------------------------------------------

|B||D|❚G❚|C|  ❚ ❚|H||A||I|  | || || |
|C||D|❚G❚|C|  ❚ ❚|H||A||I|  |F||E|| |
|B||D|❚G❚|F|  ❚ ❚|H||A||I|  |B||E|| |
|A||D|❚G❚|H|  ❚I❚|F||E||C|  |B||E||F|
------------------------------------------------

|B||D||G||C|  ❚I❚|H||A|❚ ❚  | || || |
|C||D||G||C|  ❚I❚|H||A|❚ ❚  |F||E|| |
|B||D||G||F|  ❚I❚|H||A|❚ ❚  |B||E|| |
|A||D||G||H|  ❚I❚|F||E|❚C❚  |B||E||F|
------------------------------------------------

|B||D||G|❚ ❚  |I||H||A|❚ ❚  | || || |
|C||D||G|❚ ❚  |I||H||A|❚C❚  |F||E|| |
|B||D||G|❚F❚  |I||H||A|❚C❚  |B||E|| |
|A||D||G|❚H❚  |I||F||E|❚C❚  |B||E||F|
------------------------------------------------

|B||D||G|❚ ❚  |I||H||A|| |  ❚ ❚| || |
|C||D||G|❚F❚  |I||H||A||C|  ❚ ❚|E|| |
|B||D||G|❚F❚  |I||H||A||C|  ❚B❚|E|| |
|A||D||G|❚H❚  |I||F||E||C|  ❚B❚|E||F|
------------------------------------------------

❚ ❚|D||G|| |  |I||H||A|| |  ❚ ❚| || |
❚C❚|D||G||F|  |I||H||A||C|  ❚B❚|E|| |
❚B❚|D||G||F|  |I||H||A||C|  ❚B❚|E|| |
❚A❚|D||G||H|  |I||F||E||C|  ❚B❚|E||F|
------------------------------------------------

❚ ❚|D||G|| |  |I||H||A|❚C❚  | || || |
❚ ❚|D||G||F|  |I||H||A|❚C❚  |B||E|| |
❚B❚|D||G||F|  |I||H||A|❚C❚  |B||E|| |
❚A❚|D||G||H|  |I||F||E|❚C❚  |B||E||F|
------------------------------------------------

❚ ❚|D||G|| |  |I||H||A||C|  ❚B❚| || |
❚ ❚|D||G||F|  |I||H||A||C|  ❚B❚|E|| |
❚ ❚|D||G||F|  |I||H||A||C|  ❚B❚|E|| |
❚A❚|D||G||H|  |I||F||E||C|  ❚B❚|E||F|
------------------------------------------------

❚A❚|D||G|| |  |I||H|❚ ❚|C|  |B|| || |
❚A❚|D||G||F|  |I||H|❚ ❚|C|  |B||E|| |
❚A❚|D||G||F|  |I||H|❚ ❚|C|  |B||E|| |
❚A❚|D||G||H|  |I||F|❚E❚|C|  |B||E||F|
------------------------------------------------

|A||D||G|| |  |I||H|❚E❚|C|  |B|❚ ❚| |
|A||D||G||F|  |I||H|❚E❚|C|  |B|❚ ❚| |
|A||D||G||F|  |I||H|❚E❚|C|  |B|❚ ❚| |
|A||D||G||H|  |I||F|❚E❚|C|  |B|❚ ❚|F|
------------------------------------------------

|A||D||G|❚ ❚  |I||H||E||C|  |B|| |❚ ❚
|A||D||G|❚ ❚  |I||H||E||C|  |B|| |❚F❚
|A||D||G|❚ ❚  |I||H||E||C|  |B|| |❚F❚
|A||D||G|❚H❚  |I||F||E||C|  |B|| |❚F❚
------------------------------------------------

|A||D||G|❚H❚  |I|❚ ❚|E||C|  |B|| || |
|A||D||G|❚H❚  |I|❚ ❚|E||C|  |B|| ||F|
|A||D||G|❚H❚  |I|❚ ❚|E||C|  |B|| ||F|
|A||D||G|❚H❚  |I|❚F❚|E||C|  |B|| ||F|
------------------------------------------------

|A||D||G||H|  |I|❚F❚|E||C|  |B|| |❚ ❚
|A||D||G||H|  |I|❚F❚|E||C|  |B|| |❚ ❚
|A||D||G||H|  |I|❚F❚|E||C|  |B|| |❚ ❚
|A||D||G||H|  |I|❚F❚|E||C|  |B|| |❚ ❚
------------------------------------------------
```
