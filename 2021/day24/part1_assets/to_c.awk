#!/usr/bin/env -S awk -f
BEGIN {
    print "#include <stdint.h>"
    print "int64_t monad(char digits[const 14]) {"
    print "    int i = 0;"
    print "    int64_t w = 0, x = 0, y = 0, z = 0;"
}
$1 == "inp" { print "    " $2 " = digits[i++];"; next }
$1 == "add" { print "    " $2 " += " $3 ";"; next }
$1 == "mul" { print "    " $2 " *= " $3 ";"; next }
$1 == "div" { print "    " $2 " /= " $3 ";"; next }
$1 == "mod" { print "    " $2 " %= " $3 ";"; next }
$1 == "eql" { print "    " $2 " = " $2 " == " $3 ";"; next }

# all valid lines should already have been handled.
{ print "invalid line: \"" $0 "\""; exit 1 }

END {
    print "    return z;"
    print "}"
}
