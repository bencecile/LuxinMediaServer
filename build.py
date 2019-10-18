from subprocess import Popen, run
import time

SERVER_BINARY = "luxin_media_server"
CLIENT_BINARY = "luxin_media_client"

def main():
    # Build them both first so that we can bail out if one of them fails to compile
    print("Building the server")
    run(makeBuildCommand(SERVER_BINARY), check=True)
    print("Building the client")
    run(makeBuildCommand(CLIENT_BINARY), check=True)

    with Popen(makeRunCommand(SERVER_BINARY)) as server:
        # Sleep a bit so that the server can stabilize
        time.sleep(0.2)
        with Popen(makeRunCommand(CLIENT_BINARY)) as client:
            pass

def makeBuildCommand(binary):
    return [
        "cargo", "build",
        "--bin", binary,
    ]
def makeRunCommand(binary):
    return [
        "cargo", "run",
        "--bin", binary,
    ]

if __name__ == "__main__":
    main()
