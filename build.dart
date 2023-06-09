import "dart:io";

main() {
  var r = Process.runSync("dumpbin", ["/EXPORTS", "bin/mkl_rt.2.dll"],
      runInShell: true);
  print(r.stdout.toString());
}
