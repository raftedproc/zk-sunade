import json

def dec_to_bytes32(decimal_str):
    # Convert decimal string to integer
    num = int(decimal_str)
    # Convert integer to 32-byte big-endian
    return list(num.to_bytes(32, byteorder='big'))

def print_G1Point(point, final: bool):
        x = dec_to_bytes32(point[0])
        y = dec_to_bytes32(point[1])
        print("G1Point {")
        print(f"X: U256::from_be_bytes( {x} ),")
        print(f"Y: U256::from_be_bytes( {y} ),")
        if final:
            print("};")
        else:
            print("},")

def print_named_G1Point(name, point):
        print(f"let {name} =")
        print_G1Point(point, final=True)


def print_G2Point(name, point):
        x_1 = dec_to_bytes32(point[0][1])
        x_2 = dec_to_bytes32(point[0][0])
        y_1 = dec_to_bytes32(point[1][1])
        y_2 = dec_to_bytes32(point[1][0])
        print(f"let {name} = G2Point {{")
        print(f"X: [ U256::from_be_bytes( {x_1} ), U256::from_be_bytes( {x_2}), ],")
        print(f"Y: [ U256::from_be_bytes( {y_1} ), U256::from_be_bytes( {y_2} ), ],")
        print("};")


with open("./circuits/verification_key_short.json") as f:
    data = json.load(f)
    print_named_G1Point("alfa1", data["vk_alpha_1"])
    print_G2Point("beta2", data["vk_beta_2"])
    print_G2Point("gamma2", data["vk_gamma_2"])
    print_G2Point("delta2", data["vk_delta_2"])

    print("let ic = [")
    for point in data["IC"]:
        print_G1Point(point, final=False)
    print("];")
