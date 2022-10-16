# regcmd
Tools for recording and reusing commands.  
In Linux, you can use the "up" and "down" commands, the history command, and Ctrl+R to search.  
However, it is not very effective for commands that are not the last command entered, are complex, or are used frequently.  
This tool solves these problems by saving the command to a file (~/.command).  

Aliases are advantageous for short commands, but if a shell must be created, shell file management can be complicated.  
Consolidating them into regcmd reduces management costs.　　

# Usage
## Show list
```
$ regcmd list
No    | name       | command              | description                   
0     | ll         | ls -al               | show file list                
1     | root_du    | du -h -d 1 /         | 
```

## Register
interactive
```
$ regcmd register
name: ll
command: ls -al
description: show file list
registed ll
```

option
```
$ regcmd register -n root_du -c "du -h -d 1 /"
description(option):
registed root_du
```

## Delete
```
$ regcmd delete ll
delete ll
```

# Setup
## Docker
### Linux
```
> docker build -t reccmd .
> docker run -it -v $pwd:/regcmd reccmd /bin/bash
```

### Windows
```
> docker build -t reccmd .
> docker run -it -v %cd%:/regcmd reccmd /bin/bash
```

## Build
```
$ cargo build
```
