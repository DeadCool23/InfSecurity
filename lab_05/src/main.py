import argparse

import keys as k
import sign as s
# import cipher as c

def main():
    parser = argparse.ArgumentParser(
        description="Электронная подпись"
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    gen = subparsers.add_parser("gen-keys", help="Сгенерировать RSA ключи")
    gen.add_argument("-priv", "--private", default="private.pem", help="Файл приватного ключа")
    gen.add_argument("-pub", "--public", default="public.pem", help="Файл публичного ключа")

    sign = subparsers.add_parser("sign", help="Подписать файл")
    sign.add_argument("input_file", help="Файл для подписи")
    sign.add_argument("-priv", "--private", required=True, help="Приватный ключ")
    sign.add_argument("-o", "--output", default="signature.bin", help="Файл подписи")

    verify = subparsers.add_parser("verify", help="Проверить подпись")
    verify.add_argument("input_file", help="Исходный файл")
    verify.add_argument("-pub", "--public", required=True, help="Публичный ключ")
    verify.add_argument("-s", "--signature", required=True, help="Файл подписи")

    args = parser.parse_args()

    if args.command == "gen-keys":
        k.generate_keys(args.private, args.public)
    elif args.command == "sign":
        s.sign_file(args.input_file, args.private, args.output)
    elif args.command == "verify":
        s.verify_signature(args.input_file, args.public, args.signature)

if __name__ == "__main__":
    main()
