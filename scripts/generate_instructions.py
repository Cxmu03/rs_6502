import json

with open("data/instructions.json", "r") as f:
    instructions = json.load(f)

invalid_opcodes = instructions["invalid_opcodes"]
instruction_list = instructions["instructions"]
addressing_modes = instructions["addressing_modes"]

def valid(opcode: int, instruction: str):
    mnemonic, addressing_mode = instruction.split(" ")
    return "0x{}, InstructionType::{}, AddressingMode::{}, {}, {}, Cpu::{};\n".format(
        f"{opcode:0>2X}",
        mnemonic,
        addressing_modes[addressing_mode],
        0,
        "false",
        mnemonic.lower()
    )

def invalid(opcode):
    return valid(opcode, "NOP impl")

def generate_instruction_table():
    current_valid_index = 0
    for i in range(0x100):
        if f"{i:0>2X}" in invalid_opcodes:
            yield invalid(i)
        else:
            yield valid(i, instruction_list[current_valid_index])
            current_valid_index += 1

def main():
    with open("instruction_table.txt", "w") as f:
        for instruction in generate_instruction_table():
            # ugly ass hack
            if instruction.startswith("0xFF"):
                instruction = instruction[:-2]
            f.write(instruction)

if __name__ == "__main__":
    main()