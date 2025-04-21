public interface IResult<T>
{
}

public static class Result
{
    public static Ok<T> Ok<T>(T value) => new Ok<T>(value);
    public static Err<T> Err<T>(string err) => new Err<T>(err);
}

public class Ok { }

public class Ok<T> : Ok, IResult<T>
{
    public readonly T Value;

    public Ok(T value)
    {
        Value = value;
    }
}

public class Err
{
    private readonly string _err;

    public Err(string err)
    {
        _err = err;
    }

    public Err<T> For<T>() => new Err<T>(_err);

    public override string ToString() => _err;
}

public class Err<T> : Err, IResult<T>
{
    public Err(string err) : base(err) { }
}

public static class ResultHelpers
{
    public static T Unwrap<T>(this IResult<T> result) =>
        result switch
        {
            Ok<T> ok => ok.Value,
            Err err => throw new Exception(err.ToString()),
            _ => throw new Exception("Unexpected result type"),
        };
}
