# SPDX-FileCopyrightText: 2025 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

BEGIN { 
    print "long long coprocess(void) {"
    printf "    long long a = 1"
    split("b c d e f g h", idents)
    for (i = 1; i <= 7; i++) {
        printf ", %s = 0", idents[i]
    }
    print ";"
}

# add a label before every line of source code, so that gotos can work.
{ printf "L%02d:\n    ", NR }

# actually translate the machine code to c
$1 == "set" { printf "%s = %s;\n", $2, $3 }
$1 == "sub" { printf "%s -= %s;\n", $2, $3 }
$1 == "mul" { printf "%s *= %s;\n", $2, $3 }

# add offset to current line number to get label number to jump to
$1 == "jnz" { printf "if (%s) goto L%02d;\n", $2, NR + $3 }

END { 
    printf "L%02d:\n    return h;\n", NR + 1;
    print "}"
}
