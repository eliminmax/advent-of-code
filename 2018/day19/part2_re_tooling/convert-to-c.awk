# SPDX-FileCopyrightText: 2025 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

BEGIN { 
    print "unsigned long long elfcode(void) {"
    print "    unsigned long long r0 = 1;"
    print "    unsigned long long r1 = 0, r2 = 0, r3 = 0, r4 = 0, r5 = 0;"
    line_num = 0
}

$2 == "ip" { lines[line_num++] = "return r0"; next }
$1 == "set" { lines[line_num++] = $2" = "$3; next }

$1 == "add" { $1 = "+" }
$1 == "mul" { $1 = "*" }
$1 ~ /^[+*]$/ {
    if ($2 == $3) {
        s = $2" "$1"= "$4
    } else if ($2 == $4) {
        s = $2" "$1"= "$3
    } else {
        s = $2" = "$3" "$1" "$4
    }
    lines[line_num++] = s
    next
}

$1 == "jabs" {
    lines[line_num++] = "goto *l["$2"]"
    next
}

$1 == "jrel" {
    lines[line_num++] = "goto *l["NR - 1" + "$2"]"
    next
}

$1 == "gt" {
    $1 = ">"
}
$1 == "eq" {
    $1 = "=="
}

{ lines[line_num++] = $2" = "$3" "$1" "$4 }




END { 
    printf "    void *l["NR"] = {"
    for (i = 0; i < line_num; i++) {
        printf "&&L%02d, ", i
    }
    print "};"

    for (i = 0; i < line_num; i++) {
        printf "    %s;\nL%02d:\n", lines[i], i
    }

    print "    return r0;\n}"
}
