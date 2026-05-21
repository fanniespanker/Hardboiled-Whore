import re
import sys

INLINE_TAGS = {
    "say", "think", "gesture",
    "soft", "strong", "emph",
    "redacted", "censored", "glitched"
}

SELF_CLOSING_PATTERN = re.compile(r"<(\w+)([^>]*)/>")
OPEN_TAG_PATTERN = re.compile(r"<(\w+)([^>/]*)>")
CLOSE_TAG_PATTERN = re.compile(r"</(\w+)>")

def is_self_closing(tag_text):
    return tag_text.endswith("/>")

def tokenize_lines(lines):
    tokens = []
    for i, line in enumerate(lines, start=1):
        for match in re.finditer(r"<[^>]+>", line):
            tokens.append((i, match.group()))
    return tokens

def validate_irreal_nesting(tokens):
    stack = []
    errors = []

    for line, token in tokens:
        if is_self_closing(token):
            continue

        open_match = OPEN_TAG_PATTERN.match(token)
        close_match = CLOSE_TAG_PATTERN.match(token)

        if open_match:
            tag = open_match.group(1)
            if tag == "irreal":
                if "irreal" in stack:
                    errors.append(
                        f"[irreal-nesting] line {line}: nested <irreal>"
                    )
                stack.append(tag)
            else:
                stack.append(tag)

        elif close_match:
            tag = close_match.group(1)
            if tag in stack:
                # pop until match
                while stack:
                    top = stack.pop()
                    if top == tag:
                        break

    return errors

def find_discourse_boundaries(lines):
    # blank line = discourse boundary
    boundaries = set()
    for i, line in enumerate(lines, start=1):
        if line.strip() == "":
            boundaries.add(i)
    return boundaries

def validate_inline_spans(tokens, boundaries):
    stack = []
    errors = []

    # track open inline tags: tag -> (start_line)
    open_inline = []

    for line, token in tokens:
        if is_self_closing(token):
            continue

        open_match = OPEN_TAG_PATTERN.match(token)
        close_match = CLOSE_TAG_PATTERN.match(token)

        if open_match:
            tag = open_match.group(1)
            if tag in INLINE_TAGS:
                open_inline.append((tag, line))

        elif close_match:
            tag = close_match.group(1)
            if tag in INLINE_TAGS:
                # find matching open (last)
                for i in range(len(open_inline) - 1, -1, -1):
                    if open_inline[i][0] == tag:
                        start_tag, start_line = open_inline.pop(i)

                        # check for blank lines between start_line and line
                        for b in boundaries:
                            if start_line < b < line:
                                errors.append(
                                    f"[cross-boundary] <{tag}> spans lines {start_line}-{line} across blank line {b}"
                                )
                                break
                        break

    return errors

def main(filepath):
    with open(filepath, "r", encoding="utf-8") as f:
        lines = f.readlines()

    tokens = tokenize_lines(lines)
    boundaries = find_discourse_boundaries(lines)

    irreal_errors = validate_irreal_nesting(tokens)
    inline_errors = validate_inline_spans(tokens, boundaries)

    if not irreal_errors and not inline_errors:
        print("✅ No schema violations found.")
        return

    print("❌ Schema violations:\n")

    for e in irreal_errors:
        print(e)

    for e in inline_errors:
        print(e)

def get_by_path(data, path):
    parts = path.split(".")
    cur = data
    for p in parts:
        if isinstance(cur, dict) and p in cur:
            cur = cur[p]
        else:
            return None
    return cur


def set_by_path(data, path, value):
    parts = path.split(".")
    cur = data
    for p in parts[:-1]:
        cur = cur.setdefault(p, {})
    cur[parts[-1]] = value


def collect_roots(spec):
    gc = spec.get("gc_policy", {})
    root_paths = gc.get("roots", [])

    new_spec = {}

    for path in root_paths:
        value = get_by_path(spec, path)
        if value is not None:
            set_by_path(new_spec, path, value)

    return new_spec


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python cnml_validate.py <file.cnml>")
        sys.exit(1)

    main(sys.argv[1])