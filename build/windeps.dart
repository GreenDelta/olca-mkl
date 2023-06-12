import "dart:io";

main() {
  var r = Process.runSync("Dependencies.exe", ["-imports", "bin/mkl_rt.2.dll"]);
  if (r.exitCode != 0) {
    if (r.stderr != null) {
      print("failed to get dependencies:\n${r.stderr.toString()}");
    } else {
      print("failed to get dependencies");
    }
    return;
  }

  var deps = r.stdout.toString();
  print(deps);
}
