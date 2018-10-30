name:                  mycon
version:               "0.1.X"
author:                Olaf Michaelis <o.mic@web.de>
about:                 Mycon - provides some connection information
args:
    - verbose:
        short:         v
        long:          verbose
        help:          talk to me, baby
        takes_value:   false
        multiple:      false
        required:      false
    - debug:
           short:         d
           long:          debug
           help:          debug mode, does only print what would have happened
           takes_value:   false
           multiple:      false
           required:      false
