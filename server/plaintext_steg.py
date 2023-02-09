import codecs
import numpy
import pyperclip
import sys

ENCODING = "utf8"
SECRET_ENCODING = "rot13"

mongolian_vowel_separator = "\u180e"
zero_width_space = "\u200b"
zero_width_non_joiner = "\u200c"
zero_width_joiner = "\u200d"
left_to_right_mark = "\u200e"
zero_width_no_break = "\ufeff"

hidden_chars = [
        mongolian_vowel_separator,
        left_to_right_mark,
        zero_width_no_break,
        zero_width_space,
        ]

def encode(input_str):

    global replace_p

    if input_str == "":
        plaintext = input("Please enter your plaintext: ")
    else:
        plaintext = input_str

    if len(plaintext) < 2:
        print("Plaintext is too short. Should be at least two characters.")
        exit(0)

    hidden_text = input("Please enter your hidden message: ")
    # hidden_text_bin = ''.join(format(ord(x), 'b').zfill(8) for x in hidden_text)

    hidden_text_bin = ""
    for my_byte in codecs.encode(hidden_text, SECRET_ENCODING).encode(ENCODING):
        hidden_text_bin += f"{my_byte:0>8b}"

    group_size = 2
    grouped = [hidden_text_bin[i:i+group_size] for i in range(0, len(hidden_text_bin), group_size)]

    # convert to hidden chars
    hidden = [hidden_chars[int(i, 2)] for i in grouped]
    print(f"Inserting {len(hidden)} hidden characters.")
    groups = len(plaintext) - 1
    hidden_grouped = list(numpy.array_split(numpy.array(hidden), groups))

    stegged = plaintext[0]
    for i in range(0, len(hidden_grouped)):
        next_hidden = "".join(hidden_grouped[i])
        stegged += next_hidden
        next_plain = plaintext[i+1]
        stegged += next_plain

    if replace_p:
        print("Adding <p> tags to stegged...")
        stegged = "<p>" + stegged
        stegged = "</p>\n<p>".join(stegged.splitlines())
        stegged += "</p>"

    print(f"Plaintext (len {len(plaintext)}): '{plaintext}'")
    print(f"Hidden text (len {len(hidden_text)}): '{hidden_text}'")
    print(f"Stegged text (len {len(stegged)}): '{stegged}'")
    pyperclip.copy(stegged)
    print("Copied stegged text to clipboard.")

def decode():
    print("Please copy your stegged text and press ENTER.", end=' ')
    input()

    stegged = pyperclip.paste()
    print(f"De-stegging string '{stegged}' (len {len(stegged)})")

    hidden = ""
    for i in stegged:
        if i in hidden_chars:
            hidden += i

    print(f"Read {len(hidden)} hidden chars.")

    if len(hidden) < 4:
        print("Are you sure you copied all the text?")
        return

    bin_text = ""
    for i in hidden:
        index = hidden_chars.index(i)
        next_addition = format(index, 'b').zfill(2)
        bin_text += next_addition

    stegged_text = bytes([int(bin_text[i:i+8], 2) for i in range(0, len(bin_text), 8)]).decode(ENCODING)
    stegged_text = codecs.decode(stegged_text, SECRET_ENCODING)

    print(f"Stegged text: '{stegged_text}'")

def print_help():
    print("Help menu:")
    print("Required options - either:")
    print("\t-e : encode a message")
    print("\t-d : decode a message")
    print("Optional parameters:")
    print("\t-h : print this help message")
    print("\t-i <file> : specify a plaintext input file to encode")
    print("\t-p : include HTML <p> tags. Useful for pasting to our website.")

input_str = ""
replace_p = False

if len(sys.argv) < 2 or "-h" in sys.argv:
    print_help()

if "-p" in sys.argv:
    replace_p = True

if "-i" in sys.argv and sys.argv.index("-i") < len(sys.argv) - 1:
    # read a file
    with open(sys.argv[sys.argv.index("-i") + 1], "r") as f:
        encode(f.read())
    exit(0)

elif "-e" in sys.argv:
    encode("")
elif "-d" in sys.argv:
    decode()
