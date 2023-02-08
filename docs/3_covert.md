
# Covert Channel: Plaintext Steganography
Text can be hidden within blog entries thanks to zero-width Unicode characters (original idea: [Nate Grover](https://medium.com/@SimpleDynamics/plaintext-steganography-on-the-web-c0af4ece9f58)).

Our program uses four different zero-width characters:

- the mongolian vowel separator (`U+180E`)
- the zero-width space (`U+200B`)
- the left-to-right mark (`U+200E`)
- the zero-width no-break (`U+FEFF`)

These characters are interchangable with binary using the following scheme:

- `U+180E` == `00`
- `U+200B` == `01`
- `U+200E` == `10`
- `U+FEFF` == `11`

## Encoding

Plaintext is converted character-by-character into an ASCII-encoded binary string.
Binary characters are then replaced with zero-width characters as shown above.

The list of zero-width characters is then mixed into the containing text (blog entry).
The distribution is irrelevant as long as the end-reader is able to copy the containing text along with each zero-width character.

The output of the encoding algorithm is the containing text with many zero-width characters mixed in.
This output may be posted to a public website like our blog.

## Decoding

Decoding is not much different than encoding (but in reverse).

First, zero-width characters are extracted from the input text.
English does not use any of our chosen zero-width characters, so there should be very little noise.

Next, each zero-width character is converted into two binary bits following the method defined above.

Finally, the binary string is converted character-by-character into an ASCII string and is returned as output.
