import os
import subprocess
import sys
import urllib.request as request

# From https://github.com/rust-lang/cargo/blob/9ebfb9b6d2bad7f9640ae012fae686ceeab7b084/src/etc/download.py
def run(args, quiet=False):
    if not quiet:
        print("running: " + ' '.join(args))
    sys.stdout.flush()
    # Use Popen here instead of call() as it apparently allows powershell on
    # Windows to not lock up waiting for input presumably.
    ret = subprocess.Popen(args,
                           stdin=subprocess.PIPE,
                           stdout=subprocess.PIPE,
                           stderr=subprocess.PIPE)
    out, err = ret.communicate()
    code = ret.wait()
    if code != 0:
        print("stdout: \n\n" + out)
        print("stderr: \n\n" + err)
        raise Exception("failed to fetch url")

def download(url, path):
    if not os.path.exists(path):
        print("downloading: url={}, target={}".format(url, path))
        request.urlretrieve(url, path)

def create_path(path):
    if not os.path.exists(path):
        print("creating: ", path)
        os.makedirs(path)

def urlopen(*args, **kwargs):
    print("reading: {}".format(args[0]))
    return request.urlopen(*args, **kwargs)

def install_rust(build_path, channel, triple):
    base_url = "http://static.rust-lang.org/dist"

    # Determine the name of the executable to download
    channel_url = "{}/channel-rust-{}".format(base_url, channel)
    rust_installer = None
    with urlopen(channel_url) as req:
        for line_bytes in req:
            line = line_bytes.decode(encoding="utf-8").strip()
            if line.endswith(".exe") and triple in line:
                rust_installer = line.strip()
                break

    if rust_installer == None:
        print("Invalid release: channel={}, triple={}".format(channel, triple), file=sys.stderr)
        sys.exit(1)

    # Download it
    installer_url = "{}/{}".format(base_url, rust_installer)
    target_name = "rust-{}-{}.exe".format(channel, triple)
    target_path = os.path.join(build_path, target_name)
    download(installer_url, target_path)

def main(argv):
    if len(argv) != 3:
        print("Usage: {} <channel> <triple>".format(argv[0]))
        return

    build_path = os.path.join(os.path.dirname(argv[0]), "../build")
    channel = argv[1].lower()
    triple = argv[2].lower()

    create_path(build_path)
    install_rust(build_path, channel, triple)

if __name__ == "__main__":
    main(sys.argv)
