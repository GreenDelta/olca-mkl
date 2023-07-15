// A script that copies the library build into the default folder of the openLCA
// workspace.
import 'dart:io';

main() {
  var buildDir = Directory("bin");
  if (!buildDir.existsSync()) {
    print("error: bin folder does not exist; run a build first");
    return;
  }

  var homeVar = Platform.isWindows ? "UserProfile" : "HOME";
  var homePath = Platform.environment[homeVar];
  if (homePath == null) {
    print("error: HOME variable '$homeVar' is not defined");
    return;
  }

  var home = Directory(homePath);
  if (!home.existsSync()) {
    print("error: $home does not exist");
    return;
  }

  var workspace = _subDir(home, "openLCA-data-1.4");
  var libDir = _subDir(workspace, "olca-mkl-x64_v1");
  for (var e in buildDir.listSync()) {
    var source = File(e.path);
    var name = e.path.split(Platform.pathSeparator).last;
    var target = libDir.path + Platform.pathSeparator + name;
    source.copySync(target);
  }
}

Directory _subDir(Directory parent, String name) {
  var path = parent.path + Platform.pathSeparator + name;
  var sub = Directory(path);
  if (!sub.existsSync()) {
    sub.createSync(recursive: true);
  }
  return sub;
}
