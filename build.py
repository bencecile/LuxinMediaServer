from subprocess import Popen

def main():
    with Popen(makeServerCommand()) as server:
        with Popen(makeClientCommand()) as client:
            pass

def makeServerCommand():
    return [
        "cargo", "run",
        "--bin", "luxin_media_server",
    ]
def makeClientCommand():
    return [
        "cargo", "run",
        "--bin", "luxin_media_client",
    ]

if __name__ == "__main__":
    main()
