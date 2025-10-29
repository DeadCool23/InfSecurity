from Crypto.PublicKey import RSA
from Crypto.Cipher import PKCS1_v1_5
from Crypto import Random

def encrypt(input_file, public_key_path, output_file):
    with open(public_key_path, "rb") as f:
        public_key = RSA.import_key(f.read())

    cipher = PKCS1_v1_5.new(public_key)

    with open(input_file, "rb") as f:
        data = f.read()

    key_size = public_key.size_in_bytes()
    chunk_size = key_size - 11
    
    encrypted_data = b""
    for i in range(0, len(data), chunk_size):
        chunk = data[i:i + chunk_size]
        encrypted_data += cipher.encrypt(chunk)

    with open(output_file, "wb") as f:
        f.write(encrypted_data)

    print(f"Файл '{input_file}' зашифрован в '{output_file}'")


def decrypt(input_file, private_key_path, output_file):
    with open(private_key_path, "rb") as f:
        private_key = RSA.import_key(f.read())

    cipher = PKCS1_v1_5.new(private_key)

    with open(input_file, "rb") as f:
        encrypted_data = f.read()

    key_size = private_key.size_in_bytes()
    decrypted_data = b""

    sentinel = Random.new().read(15)

    for i in range(0, len(encrypted_data), key_size):
        chunk = encrypted_data[i:i + key_size]
        decrypted_data += cipher.decrypt(chunk, sentinel)

    with open(output_file, "wb") as f:
        f.write(decrypted_data)

    print(f"Файл '{input_file}' расшифрован в '{output_file}'")
