import re

with open("input.asm", "r") as f:
    lines = f.readlines()

# Match any label definition: beginning of line, optional spaces, word (including dots), :
label_pattern = re.compile(r'^\s*([A-Za-z_.][\w.]*)\s*:')

labels_in_order = []
for i, line in enumerate(lines):
    matches = label_pattern.findall(line)
    labels_in_order.extend(matches)

# skip numbers with only 0 and 1
def gen_local_label():
    label = 2
    while True:
        if set(str(label)) <= {'0', '1'}:
            label += 1
            continue
        yield label
        label += 1

number_gen = gen_local_label()
label_map = {label: str(next(number_gen)) for label in labels_in_order}

# Replace labels and jumps
def replace_line(line, line_index):
    # Replace all label definitions in the line
    for label in label_map:
        line = re.sub(
            rf'^(\s*){re.escape(label)}:',
            lambda m: f"{m.group(1)}{label_map[label]}:",
            line
        )

    # Replace jumps to labels
    def replace_jump(match):
        instr, target = match.groups()
        if target in label_map:
            target_num = label_map[target]
            # Decide forward/backward
            for i, l in enumerate(lines[:line_index]):
                if label_pattern.match(l) and label_pattern.match(l).group(1) == target:
                    return f"{instr} {target_num}b"
            return f"{instr} {target_num}f"
        return match.group(0)

    # match jump [spaces] [label]
    line = re.sub(
        r'\b(jmp|je|jne|jg|jl|jge|jle|ja|jb|jae|jbe|js)\s+([A-Za-z_.][\w.]*)',
        replace_jump,
        line
    )
    return line

new_lines = [replace_line(line, i) for i, line in enumerate(lines)]

with open("output.asm", "w") as f:
    f.writelines(new_lines)

print("All labels replaced safely. Output saved to output.asm")
