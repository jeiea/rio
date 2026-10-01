const installPath = Deno.env.get("RIO_INSTALL_PATH");
if (!installPath) {
    throw new Error(
        "Set RIO_INSTALL_PATH to the installed rio.exe path before building.",
    );
}
if (!(await Deno.stat(installPath)).isFile) {
    throw new Error(`Installed executable not found: ${installPath}`);
}

const build = await new Deno.Command("cargo", {
    args: [
        "build",
        "-p",
        "rioterm",
        "--release",
        "--message-format=json-render-diagnostics",
    ],
    cwd: new URL(".", import.meta.url),
    stdout: "piped",
    stderr: "inherit",
}).output();

if (!build.success) {
    Deno.exit(build.code);
}

const executable = new TextDecoder().decode(build.stdout)
    .trim().split("\n").map((line) => JSON.parse(line))
    .findLast((message) =>
        message.reason === "compiler-artifact" &&
        message.target.name === "rio" && message.executable
    )?.executable;
if (!executable) {
    throw new Error("Cargo did not report a rio executable.");
}

await Deno.copyFile(executable, installPath);
console.log(`Installed ${executable} -> ${installPath}`);
