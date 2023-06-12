import "dart:io";
import "./build/winlib.dart" as winlib;

main() {
  if (Platform.isWindows) {
    winlib.generateLibFile();
  }
  var r = Process.runSync("cargo", ["build", "--release"],
      environment: {"RUSTFLAGS": "-C target-feature=+crt-static"});

  if (r.exitCode != 0) {
    if (r.stderr != null) {
      print("build failed:\n${r.stderr.toString()}");
    } else {
      print("build failed no error message present");
    }
    return;
  }
  if (r.stdout != null) {
    print(r.stdout.toString());
  }
}
