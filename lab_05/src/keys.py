from Crypto.PublicKey import RSA

def generate_keys(private_key_path, public_key_path):
    key = RSA.generate(2048)
    private_key = key.export_key()
    public_key = key.publickey().export_key()

    with open(private_key_path, "wb") as f:
        f.write(private_key)
    with open(public_key_path, "wb") as f:
        f.write(public_key)

    print(f"Ключи сгенерированы:\n - Приватный: {private_key_path}\n - Публичный: {public_key_path}")
