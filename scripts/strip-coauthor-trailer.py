import sys
text = sys.stdin.read()
lines = [
    l
    for l in text.splitlines(keepends=True)
    if "Co-authored-by: Cursor" not in l and "cursoragent@cursor.com" not in l
]
sys.stdout.write("".join(lines))
