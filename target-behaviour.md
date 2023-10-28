undertale
---------

```bash
lynback save init
lynback save pacifist
lynback save tori -m "progressed until toriel"
lynback save 'kill me'
lynback load init
lynback save murderer
lynback save -m "killed toriel muahaha"
lynback save
lynback save undyne
lynback load murderer
lynback save spare-tori -m "we kill everyone in ruins; we then spare toriel"
lynback load murderer
lynback save doge
```

```tree
# largest layout
0x001a0b init  
│
├╴0x2084B1         pacifist
│ │
│ ╰╴0x3029AE       tori         # progressed until toriel
│   │
│   ╰╴0x3029AE     kill me
│
╰╴0x9C109F         murderer
  │
  ├╴0x1900C3                    # killed toriel muahaha
  │ │
  │ ╰╴0x309FA0
  │   │
  │   ╰╴0x90289E   undyne
  │
  ├╴0x90289E       spare-tori   # we kill everyone in ruins; we then spare toriel
  │
  ╰╴0x102948       doge


# compact layout
0x001a0b init  
├╴0x2084B1      pacifist
│ ╰╴0x3029AE    tori
│ │             # progressed until toriel
│ ╰╴0x3029AE    kill me
╰╴0x9C109F      murderer
  ├╴0x1900C3    # killed toriel muahaha
  │ ╰╴0x309FA0  
  │ ╰╴0x90289E  undyne
  ├╴0x90289E    spare-tori
  │             # we kill everyone in ruins
  │             # we then spare toriel
  ╰╴0x102948    doge

# - empty lines are removed
# - only two spaces as separator
# - messages appear below names
#   if no name exists, they take its place
# - messages are split on '\n', '.', ',', ';'
#   also on any whitespace after a threshhold
#   trailing whitespace or ';' is not printed
# - saves without siblings are shifted left:
#
#   Large         Compact
#
#   0x000000      0x000000
#   │             ╰╴0x000000
#   ╰╴0x000000    ╰╴0x000000
#     │
#     ╰╴0x000000
```
