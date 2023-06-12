import "dart:io";
import "./build/winlib.dart" as winlib;

main() {
  if (Platform.isWindows) {
    winlib.generateLibFile();
  }

  print("compile library ...");
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

  var build = File("target/release/olcamkl.dll");
  if (!build.existsSync()) {
    print("ERROR: build failed; ${build.path} does not exist");
    return;
  }
  build.copySync("bin/olcamkl.dll");
  build.deleteSync();
}
