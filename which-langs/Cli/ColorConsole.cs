using leConsole = System.Console;

public static class Console
{
    public static void WriteLine(string msg) =>
        leConsole.WriteLine(msg);

    public static void WriteError(string msg) =>
        WriteWithColor(msg, fg: ConsoleColor.Red, bg: null);

    public static void WriteError(Err err) =>
        WriteWithColor(err.ToString(), fg: ConsoleColor.Red, bg: null);

    public static void WriteWarn(string msg) =>
        WriteWithColor(msg, fg: ConsoleColor.Yellow, bg: null);

    private static void WriteWithColor(string msg, ConsoleColor? fg, ConsoleColor? bg)
    {
        try
        {
            if (fg != null)
                leConsole.ForegroundColor = fg.Value;
            if (bg != null)
                leConsole.BackgroundColor = bg.Value;
            WriteLine(msg);
        }
        finally
        {
            leConsole.ResetColor();
        }
    }
}

