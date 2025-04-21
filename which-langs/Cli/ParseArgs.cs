public static class ParseArgs
{
    public static IResult<Options> ParseAndValidate(string[] args)
    {
        if (args.Any(arg => arg is "-?" or "--help" or "-h"))
        {
            Console.WriteLine(Options.Usage());
            Environment.Exit(0);
        }


        var targetPath = args.LastOrDefault(arg => arg[0] != '-') ?? ".";
        var result = IO.TryExtractFullPath(targetPath);
        if (result is Err error)
        {
            return error.For<Options>();
        }

        var verbose =
            args.Any(arg => arg is "-vv" or "--debug") ? Verbosity.Debug :
            args.Any(arg => arg is "-v" or "--verbose") ? Verbosity.Verbose :
            Verbosity.Always
        ;

        return Result.Ok(new Options(result.Unwrap(), verbose));
    }
}

public class Options
{
    public string FullPathToTargetRepo { get; }

    public Verbosity SelectedVerbosity { get; }

    public Options(string fullPathToTargetRepo, Verbosity verbose) =>
        (FullPathToTargetRepo, SelectedVerbosity) = (fullPathToTargetRepo, verbose);

    public static string Usage() =>
        """
        which-langs
          detects the programming languages used in a repository

        Usage:
          which-langs [OPTIONS?] [PATH?]

        Options:
          --version       Show version information
          -v, --verbose   Verbose log
          -vv, --debug    Chatty verbose log for DEBUG purposes
          -?, -h, --help  Show help and usage information
        """;

    internal Action<string>? Log(Verbosity verbosity, Action<string> consoleLog)
    {
        if (SelectedVerbosity < verbosity)
            return null;

        return consoleLog;
    }
}

public enum Verbosity
{
    Always,
    Verbose,
    Debug,
}
