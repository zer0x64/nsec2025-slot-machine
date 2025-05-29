# Taken and modified from
# from https://github.com/yinengy/Mersenne-Twister-in-Python/blob/master/MT19937.py

# coefficients for MT19937
from binascii import unhexlify
import struct

# Constants
(w, n, m, r) = (32, 624, 397, 31)
a = 0x9908B0DF
(u, d) = (11, 0xFFFFFFFF)
(s, b) = (7, 0x9D2C5680)
(t, c) = (15, 0xEFC60000)
l = 18
f = 1812433253

lower_mask = 0x7FFFFFFF #(1 << r) - 1 // That is, the binary number of r 1's
upper_mask = 0x80000000 #lowest w bits of (not lower_mask)

# Global state of the RNG
MT = []
index = 0

# Extract a tempered value based on MT[index]
# calling twist() every n numbers
def random():
    global index
    if index >= n:
        twist()
        index = 0

    y = MT[index]
    y = y ^ ((y >> u) & d)
    y = y ^ ((y << s) & b)
    y = y ^ ((y << t) & c)
    y = y ^ (y >> l)

    index += 1
    return y & 0xffffffff

# Generate the next n values from the series x_i
def twist():
    for i in range(0, n):
        x = (MT[i] & upper_mask) + (MT[(i+1) % n] & lower_mask)
        xA = x >> 1
        if (x % 2) != 0:
            xA = xA ^ a
        MT[i] = MT[(i + m) % n] ^ xA

# Get the symbols corresponding to the RNG output
def symbol_from_num(num):
    if num < 128:
        return "Cherry"
    elif num < 192:
        return "Bar"
    elif num < 218:
        return "DoubleBar"
    elif num < 240:
        return "TripleBar"
    elif num < 249:
        return "Seven"
    elif num < 253:
        return "MinorJackpot"
    elif num < 255:
        return "MajorJackpot"
    elif num == 255:
        return "GrandJackpot"

def main():
    global MT, index

    # Read debug output from file
    state_encoded = ""
    with open("flag3_debug_buffer.txt", "r") as f:
        state_encoded = f.read().strip("\n")

    # Decode numbers from hex
    state = []
    for i in range(0, len(state_encoded), 8):
        num = unhexlify(state_encoded[i:i + 8])
        # <I means little endian unsigned int(32 bits)
        (num,) = struct.unpack("<I", num)
        state.append(num)

    # The first part of the debug output can be used to get the state
    MT = state[:624]

    # We untemper the output here, see another attack on MT19937:
    # https://github.com/zer0x64/cryptopals-rs/blob/master/set3/chal23/src/main.rs
    for i in range(0, len(MT)):
        MT[i] ^= MT[i] >> l
        MT[i] ^= (MT[i] << t) & c

        intermediate = 0
        for y in range(0, 32 // s + 1):
            mask = ((1 << s) - 1) << (s * y)
            intermediate |= (MT[i] ^ ((intermediate << s) & b)) & mask

        MT[i] = intermediate

        intermediate = 0
        for y in range(0, 32 // u + 1):
            mask = ((1 << u) - 1) << (u * (32 // u - y))
            intermediate |= (MT[i] ^ (intermediate >> u)) & mask

        MT[i] = intermediate

    # Sets the index high to trigger a twist on next fetch
    index = 0xFFFFFFFF

    # Check with the rest of the state to make sure the RNG has been cloned well
    for i in range(len(MT), len(state)):
        if random() != int(state[i]):
            print("debug input doesn't match at offset %d!" % i)
            exit()


    # Here we predict the future to find winning spins
    print("RNG cloned!")
    print("All in on the following spins:")

    num_wins = 0
    n_spin = 0
    while num_wins < 10:
        n_spin += 1

        # When we want a byte, the random spits out a u32, but we only need the first 8 bits
        x = symbol_from_num(random() & 0xFF)
        y = symbol_from_num(random() & 0xFF)
        z = symbol_from_num(random() & 0xFF)

        # Found a winning spin!
        if x == y == z:
            print("Spin %d: %s" % (n_spin, x))
            num_wins += 1

if __name__ == '__main__':
    main()
