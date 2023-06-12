// This module generates a `lib` file on Windows that is required for the
// linker. See https://stackoverflow.com/a/16127548/599575

import "dart:io";

const LIB = "mkl_rt.2";

void generateLibFile() {
  var libFile = _fileOf(Directory("bin"), "$LIB.lib");
  if (libFile.existsSync()) {
    return;
  }
  var defFile = _fileOf(Directory("bin"), "${LIB}.def");
  if (!defFile.existsSync()) {
    _makeDefFile().copySync(defFile.path);
  }
  Process.runSync("lib", ["/def:${LIB}.def", "/out:${LIB}.lib", "/machine:X64"],
      runInShell: true, workingDirectory: "./bin");
  defFile.deleteSync();
}

File _makeDefFile() {
  var defFile = _fileOf(Directory("target"), "${LIB}.def");
  if (defFile.existsSync()) {
    return defFile;
  }

  print("generate export definitions: ${defFile.path}");
  var r = Process.runSync("dumpbin", ["/EXPORTS", "bin/mkl_rt.2.dll"],
      runInShell: true);
  var dump = r.stdout.toString();

  var exports = "EXPORTS\n";
  bool isInFunctionList = false;
  for (var line in dump.split("\n")) {
    var elems = line.trim().split(RegExp("\\s+"));
    if (elems.length < 4) {
      continue;
    }
    if (isInFunctionList) {
      exports += elems[3].trim() + "\n";
      continue;
    }
    if (elems[0].trim() == "ordinal" &&
        elems[1].trim() == "hint" &&
        elems[2].trim() == "RVA" &&
        elems[3].trim() == "name") {
      isInFunctionList = true;
    }
  }

  defFile.writeAsString(exports);
  return defFile;
}

File _fileOf(Directory dir, String name) {
  if (!dir.existsSync()) {
    dir.createSync(recursive: true);
  }
  return File(dir.path + Platform.pathSeparator + name);
}
