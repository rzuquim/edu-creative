var maybeOptions = ParseArgs.ParseAndValidate(args);


if (maybeOptions is Err error)
{
    Console.WriteError(error);
    Environment.Exit(-1);
}

var options = maybeOptions.Unwrap();

var perLangPercentage = DetectLang.ProbeFiles(options);

foreach (var (lang, percentage) in perLangPercentage.OrderByDescending(kvp => kvp.Value))
{

    Console.WriteLine($"{percentage * 100:00.00}% {lang}");
}
