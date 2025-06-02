# SPDX-FileCopyrightText: 2025 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

BEGIN { 
    line_num = 0
}

$2 == "ip" { lines[line_num++] = "return r0"; next }
$1 == "set" { lines[line_num++] = $2" = "$3; next }

# common handling for both bitwise and arithmetic binary operations
function basic_op(op) {
    if ($2 == $3) {
        lines[line_num++] = sprintf("%s %s= %s", $2, op, $4)
    } else if ($2 == $4) {
        lines[line_num++] = sprintf("%s %s= %s", $2, op, $3)
    } else {
        lines[line_num++] = sprintf("%s = %s %s %s", $2, $3, op, $4)
    }
    next
}

$1 == "add" { basic_op("+") }
$1 == "mul" { basic_op("*") }
$1 == "ban" { basic_op("&") }
$1 == "bor" { basic_op("|") }

$1 == "jabs" {
    lines[line_num++] = sprintf("goto L%02d", $2)
    next
}

$1 == "jrel" {
    if ($2 ~ /^[0-9]+$/) {
        lines[line_num++] = sprintf("goto L%02d", NR - 1 + $2)
    } else {
        lines[line_num++] = "goto *l["NR - 1" + "$2"]"
    }
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
    print "unsigned long long elfcode(unsigned long long r0) {"
    print "    const void *NULLP = 0;"
    print "    unsigned long long r1 = 0, r2 = 0, r3 = 0, r4 = 0, r5 = 0;"
    printf "    const void *l[] = {"
    for (i = 0; i < line_num; i++) {
        printf "&&L%02d, ", i
    }
    print "};"

    for (i = 0; i < line_num; i++) {
        printf "    %s;\nL%02d:\n", lines[i], i
    }

    print "    return r0;"
    print "}"
}
