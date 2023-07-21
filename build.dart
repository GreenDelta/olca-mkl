import "dart:io";
import "./build/winlib.dart" as winlib;
import "./build/deps.dart" as deps;

main() async {
  await deps.fetch();

  if (Platform.isWindows) {
    winlib.generateLibFile();
  }
  if (Platform.isLinux) {
    var binDir = Directory("./bin");
    if (Platform.isLinux) {
      var lnFile = File(binDir.path + "/libmkl_rt.so.2");
      var lnLink = File(binDir.path + "/libmkl_rt.so");
      if (lnFile.existsSync() && !lnLink.existsSync()) {
        print("create symlink libmkl_rt.so");
        var r = Process.runSync("ln", ["-s", "libmkl_rt.so.2", "libmkl_rt.so"],
            workingDirectory: binDir.path);
        if (r.exitCode != 0) {
          print("  failed: ${r.stderr}");
        }
      }
    }
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
