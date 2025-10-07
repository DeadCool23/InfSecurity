#!/usr/bin/env python3

import os
import rsa
import argparse

parser = argparse.ArgumentParser(description="CLI для RSA шифрования")
subparsers = parser.add_subparsers(dest="command", required=True)

keygen_parser = subparsers.add_parser("keygen", help="Генерация ключей")
keygen_parser.add_argument(
    "-l", "--length",
    type=int,
    default=1024,
    help="Длина ключа (по умолчанию 1024)"
)
keygen_parser.add_argument(
    "-d", "--dir",
    default="keys",
    help="Папка для сохранения ключей (по умолчанию 'keys')"
)

encrypt_parser = subparsers.add_parser("encrypt", help="Шифрование файла")
encrypt_parser.add_argument("input_file", help="Файл для шифрования")
encrypt_parser.add_argument("output_file", help="Файл с зашифрованными данными")
encrypt_parser.add_argument("-k", "--key", default=".rsa.pub", help="Путь к публичному ключу (по умолчанию .rsa.pub)")

decrypt_parser = subparsers.add_parser("decrypt", help="Дешифрование файла")
decrypt_parser.add_argument("input_file", help="Зашифрованный файл")
decrypt_parser.add_argument("output_file", help="Файл с расшифрованными данными")
decrypt_parser.add_argument("-k", "--key", default=".rsa.priv", help="Путь к приватному ключу (по умолчанию .rsa.priv)")

if __name__ == '__main__':
    args = parser.parse_args()

    if args.command == "keygen":
        rsa.KEY_SIZE = args.length
        res = rsa.generate_keys()
        if res is None:
            exit()
        
        dir_path = args.dir
        os.makedirs(dir_path, exist_ok=True)

        pub_key, priv_key = res
        rsa.save_keys(
            pub_key,
            priv_key,
            fpub_key=f"{dir_path}/{rsa.STD_PUB_KEY_FILE}",
            fpriv_key=f"{dir_path}/{rsa.STD_PRIV_KEY_FILE}"
        )
        print(f"Ключи успешно сохранены в папку '{dir_path}'")
    elif args.command == "encrypt":
        rsa.encrypt_file(args.input_file, args.output_file, args.key)
    elif args.command == "decrypt":
        rsa.decrypt_file(args.input_file, args.output_file, args.key)
