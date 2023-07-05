import "dart:io";
import "./build/winlib.dart" as winlib;
import "./build/deps.dart" as deps;

main() async {
  await deps.fetch();

  if (Platform.isWindows) {
    winlib.generateLibFile();
  }

  print("compile library ...");
  var flags = Platform.isWindows
      ? "-C target-feature=+crt-static"
      : "-C target-feature=-crt-static";
  var r = Process.runSync("cargo", ["build", "--release"],
      environment: {"RUSTFLAGS": flags});

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

  var lib = _target();
  var build = File("target/release/$lib");
  if (!build.existsSync()) {
    print("ERROR: build failed; ${build.path} does not exist");
    return;
  }
  build.copySync("bin/$lib");
  build.deleteSync();
}

String _target() {
  if (Platform.isWindows) {
    return "olcamkl.dll";
  }
  return Platform.isMacOS ? "libolcamkl.dylib" : "libolcamkl.so";
}
