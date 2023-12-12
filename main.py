import rust
import eth_utils

def eip55_address(addr): # Takes a 20-byte binary address as input
    hex_addr = addr.hex()
    checksummed_buffer = ""

    # Treat the hex address as ascii/utf-8 for keccak256 hashing
    hashed_address = eth_utils.keccak(text=hex_addr).hex()

    # Iterate over each character in the hex address
    for nibble_index, character in enumerate(hex_addr):

        if character in "0123456789":
            # We can't upper-case the decimal digits
            checksummed_buffer += character
        elif character in "abcdef":
            # Check if the corresponding hex digit (nibble) in the hash is 8 or higher
            hashed_address_nibble = int(hashed_address[nibble_index], 16)
            if hashed_address_nibble > 7:
                checksummed_buffer += character.upper()
            else:
                checksummed_buffer += character
        else:
            raise eth_utils.ValidationError(
                f"Unrecognized hex character {character!r} at position {nibble_index}"
            )

    return "0x" + checksummed_buffer


private_key = input("Insert private key: ")
nonce = input("Insert nonce: ")
address_classic = rust.generate_addr(private_key)
address_bytes = eth_utils.to_bytes(hexstr=address_classic)
address_eip55 = eip55_address(address_bytes)
contract_address = rust.calculate_contract_address(address_classic, int(nonce))
print("Classic address: ", address_classic)
print("EIP55 address: ", address_eip55)
print("Contract address: ", contract_address)