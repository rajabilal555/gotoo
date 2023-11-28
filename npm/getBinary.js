const { Binary } = require("binary-install");
const os = require("os");
const { join } = require("path");
const packageInfo = require("../package.json");

function getPlatform() {
  const type = os.type();
  const arch = os.arch();

  if (type === "Windows_NT" && arch === "x64") return "win64";
  if (type === "Windows_NT") return "win32";
  if (type === "Linux" && arch === "x64") return "linux";
  if (type === "Darwin" && arch === "x64") return "macos";

  throw new Error(
    `Unsupported platform: ${type} ${arch}. Please create an issue at ${packageInfo.bugs.url}`
  );
}

function getBinary() {
  const platform = getPlatform();
  const version = packageInfo.version;
  const url = `https://github.com/rajabilal555/gotoo/releases/download/v${version}/gotoo-${platform}.tar.gz`;
  return new Binary("gotoo", url, version, {
    installDirectory: join(__dirname),
  });
}

module.exports = getBinary;
