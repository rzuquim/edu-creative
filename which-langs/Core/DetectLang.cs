using Lang = System.String;
using static Verbosity;

public static class DetectLang
{
    internal static Dictionary<Lang, float> ProbeFiles(Options options)
    {
        var statistics = new Statistics();
        ClassifyFilesPerExtension(options.FullPathToTargetRepo, options, statistics);
        ClassifyDirectoryPerExtension(options.FullPathToTargetRepo, options, statistics);
        return statistics.PercentagePerLang(options);
    }

    private static void ClassifyFilesPerExtension(string dir, Options options, Statistics statistics)
    {
        foreach (var file in Directory.EnumerateFiles(dir))
        {
            // TODO: detect per file name
            var lang = ClassifyPerExtension(file, out var extension);

            if (!string.IsNullOrEmpty(extension))
                options.Log(Debug, Console.WriteLine)?.Invoke($"per extension: {file} -> {lang} ({extension})");

            statistics.Process(lang, file);
        }
    }

    private static void ClassifyDirectoryPerExtension(string dir, Options options, Statistics statistics)
    {
        foreach (var subDir in Directory.EnumerateDirectories(dir))
        {
            var dirName = Path.GetFileName(subDir);
            if (IO.ShouldIgnore(dirName))
            {
                options.Log(Verbosity.Verbose, Console.WriteWarn)?.Invoke($"Ignoring dir {subDir}");
                continue;
            }

            ClassifyFilesPerExtension(subDir, options, statistics);
            ClassifyDirectoryPerExtension(subDir, options, statistics);
        }
    }

    private static Lang ClassifyPerExtension(string file, out string? extension)
    {
        extension = Path.GetExtension(file);
        if (Langs.Relevant.PerExtension.TryGetValue(extension, out var lang))
        {
            return lang;
        }

        return Langs.Others;
    }
}

