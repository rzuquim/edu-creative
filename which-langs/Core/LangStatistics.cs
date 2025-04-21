using Lang = System.String;
using static Verbosity;

public class Statistics
{
    public int TotalFilesCount { get; private set; }

    public Dictionary<Lang, LangStatistics> PerLang { get; set; } = new();

    public void Process(Lang lang, string file)
    {
        TotalFilesCount++;

        if (!PerLang.TryGetValue(lang, out var langStatistics))
            PerLang[lang] = langStatistics = new LangStatistics();
        langStatistics.Process(file);
    }

    public Dictionary<Lang, float> PercentagePerLang(Options options)
    {
        var projPercentage = new Dictionary<Lang, float>();
        foreach (var (lang, perLangStats) in PerLang)
        {
            if (projPercentage.TryGetValue(lang, out var langStats))
                projPercentage[lang] = 0f;
            projPercentage[lang] = langStats + perLangStats.FilesCount;
        }

        options.Log(Verbose, Console.WriteLine)?.Invoke($"Total files count: {TotalFilesCount}");

        foreach (var (lang, val) in projPercentage)
        {
            options.Log(Verbose, Console.WriteLine)?.Invoke($"{lang}: {val} files");
            projPercentage[lang] = val / TotalFilesCount;
        }
        return projPercentage;
    }
}

public class LangStatistics
{
    public int FilesCount { get; private set; }

    public void Process(Lang file)
    {
        FilesCount++;
    }
}
