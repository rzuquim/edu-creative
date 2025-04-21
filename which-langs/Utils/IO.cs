using FullPath = System.String;
using static Result;

public static class IO
{
    internal static IResult<FullPath> TryExtractFullPath(string targetPath)
    {
        var fullPath = Path.GetFullPath(targetPath);

        if (File.Exists(fullPath))
            return Err<FullPath>($"Please inform a directory. {fullPath} is a file.");

        if (!Directory.Exists(fullPath))
            return Err<FullPath>($"Target path {fullPath} does not exist!");

        return Ok(fullPath);
    }

    internal static bool ShouldIgnore(string dirName) =>
        _ignoredDirs.Any(ignore =>  string.Equals(ignore, dirName, StringComparison.OrdinalIgnoreCase));

    private static readonly string[] _ignoredDirs = {
        ".git",
        ".dotnet",
        "packages",
        "artifacts",
        ".idea",
        "bin",
        "obj",
        ".deps",
        ".vs",
        "publish",
        ".libs",
        "TestResults",
        "BuildLog",
        ".vscode",
        "target",
        ".mvn",
        ".nuget",
        "log",
        "logs",
        "__pycache__",
        "vendor",
        ".bundle",
        "tmp",
        ".ruby-version",
        "node_modules",
        "dist",
        ".dist",
        "coverage",
        "typings",
    };
}
