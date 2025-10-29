from Crypto.PublicKey import RSA
from Crypto.Signature import pkcs1_15
from Crypto.Hash import SHA256

def sign_file(input_file, private_key_path, signature_file):
    with open(private_key_path, "rb") as f:
        private_key = RSA.import_key(f.read())

    with open(input_file, "rb") as f:
        data = f.read()

    h = SHA256.new(data)
    signature = pkcs1_15.new(private_key).sign(h)

    with open(signature_file, "wb") as f:
        f.write(signature)
    
    print(f"Подпись сохранена в {signature_file}")

def verify_signature(input_file, public_key_path, signature_file):
    with open(public_key_path, "rb") as f:
        public_key = RSA.import_key(f.read())

    with open(input_file, "rb") as f:
        data = f.read()
    with open(signature_file, "rb") as f:
        signature = f.read()

    h = SHA256.new(data)
    try:
        pkcs1_15.new(public_key).verify(h, signature)
        print("Подпись действительна.")
    except (ValueError, TypeError):
        print("Подпись недействительна!")