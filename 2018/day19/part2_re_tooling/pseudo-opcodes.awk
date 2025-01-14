# SPDX-FileCopyrightText: 2025 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

NR == 1 { ip_reg = "r"$2; next }

function operand(o) {
    return (o == ip_reg) ? NR - 2 : o
}

# prefix destination register
{sub(/^/, "r", $4)}

/^set/ {
    if ($1 ~ /^...r/) { sub(/^/, "r", $2) }
    if ($4 == ip_reg) {
        if ($2 == ip_reg) {
            print "nop"
        } else {
            printf "jabs %s\n", $2
        }
    } else {
        printf "set %s %s\n", $4, operand($2)
    }
    next
}

/^add/ {
    if ($1 ~ /^...r/)  sub(/^/, "r", $3)
    sub(/.$/, "", $1)
    sub(/^/, "r", $2)
    if ($4 == ip_reg) {
        if ($2 == ip_reg) {
            printf "jrel %s\n", operand($3)
        } else if ($3 == ip_reg) {
            printf "jrel %s\n", operand($2)
        } else {
            printf "add ip %s %s\n", operand($2), operand($3)
        }
        next
    }
}

/^(mul|ban|bor)/ {
    if ($1 ~ /^...r/)  sub(/^/, "r", $3)
    sub(/.$/, "", $1)
    sub(/^/, "r", $2)
}

/^(add|mul|ban|bor)/ {
    dest = ($4 == ip_reg) ? "ip" : $4
    $4 = operand($3)
    $3 = operand($2)
    $2 = dest
    print
    next
}

/^..r./ {
    sub(/^/, "r", $2)
}

/^...r/ {
    sub(/^/, "r", $3)
}

{
    sub(/..$/, "", $1)
    dest = $4
    $4 = operand($3)
    $3 = operand($2)
    $2 = dest
    print
}
