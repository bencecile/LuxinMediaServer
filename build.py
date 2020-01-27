from subprocess import CalledProcessError, Popen, run
import time

SERVER_BINARY = "luxin_media_server"
CLIENT_BINARY = "luxin_media_pc_client"

def main():
    # Build them both first so that we can bail out if one of them fails to compile
    print("Building everything")
    try:
        run(["cargo", "build", "--workspace"], check=True)
    except CalledProcessError:
        return

    with Popen(makeRunCommand(SERVER_BINARY)) as server:
        # Sleep a bit so that the server can stabilize
        time.sleep(0.5)
        with Popen(makeRunCommand(CLIENT_BINARY)) as client:
            pass

def makeRunCommand(binary):
    return [f"./target/debug/{binary}.exe"]

if __name__ == "__main__":
    main()
