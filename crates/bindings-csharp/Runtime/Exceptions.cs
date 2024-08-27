namespace SpacetimeDB;

using SpacetimeDB.Internal;

public abstract class StdbException : Exception
{
    public abstract override string Message { get; }
}

public class NotInTransactionException : StdbException
{
    public override string Message => "ABI call can only be made while in a transaction";
}

public class NoSuchTableException : StdbException
{
    public override string Message => "No such table";
}

public class UniqueAlreadyExistsException : StdbException
{
    public override string Message => "Value with given unique identifier already exists";
}

public class BufferTooSmallException : StdbException
{
    public override string Message => "The provided buffer is not large enough to store the data";
}

public class NoSuchIterException : StdbException
{
    public override string Message => "The provided row iterator does not exist";
}

public class NoSuchBytesException : StdbException
{
    public override string Message => "The provided bytes source or sink does not exist";
}

public class NoSpaceException : StdbException
{
    public override string Message => "The provided bytes sink has no more room left";
}

public class UnknownException : StdbException
{
    private readonly Errno code;

    internal UnknownException(Errno code) => this.code = code;

    public override string Message => $"SpacetimeDB error code {code}";
}
