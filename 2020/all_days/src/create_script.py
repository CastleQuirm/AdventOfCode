import sys

f = open("day" + sys.argv[1] + ".rs", "w")
f.write("pub fn day" + sys.argv[1] + "(input_lines: &[String]) -> (u64, u64) {\n")
f.write("    (0, 0)\n")
f.write("}\n")
f.close()