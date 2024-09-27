﻿//HintName: TestAutoIncNotInteger.cs
// <auto-generated />
#nullable enable

partial struct TestAutoIncNotInteger : SpacetimeDB.Internal.ITable<TestAutoIncNotInteger>
{
    public void ReadFields(System.IO.BinaryReader reader)
    {
        AutoIncField = BSATN.AutoIncField.Read(reader);
        IdentityField = BSATN.IdentityField.Read(reader);
    }

    public void WriteFields(System.IO.BinaryWriter writer)
    {
        BSATN.AutoIncField.Write(writer, AutoIncField);
        BSATN.IdentityField.Write(writer, IdentityField);
    }

    public readonly partial struct BSATN : SpacetimeDB.BSATN.IReadWrite<TestAutoIncNotInteger>
    {
        internal static readonly SpacetimeDB.BSATN.F32 AutoIncField = new();
        internal static readonly SpacetimeDB.BSATN.String IdentityField = new();

        public TestAutoIncNotInteger Read(System.IO.BinaryReader reader) =>
            SpacetimeDB.BSATN.IStructuralReadWrite.Read<TestAutoIncNotInteger>(reader);

        public void Write(System.IO.BinaryWriter writer, TestAutoIncNotInteger value)
        {
            value.WriteFields(writer);
        }

        public SpacetimeDB.BSATN.AlgebraicType GetAlgebraicType(
            SpacetimeDB.BSATN.ITypeRegistrar registrar
        ) =>
            registrar.RegisterType<TestAutoIncNotInteger>(
                _ => new SpacetimeDB.BSATN.AlgebraicType.Product(
                    new SpacetimeDB.BSATN.AggregateElement[]
                    {
                        new(nameof(AutoIncField), AutoIncField.GetAlgebraicType(registrar)),
                        new(nameof(IdentityField), IdentityField.GetAlgebraicType(registrar))
                    }
                )
            );
    }

    static IEnumerable<SpacetimeDB.Internal.TableDesc> SpacetimeDB.Internal.ITable<TestAutoIncNotInteger>.MakeTableDesc(
        SpacetimeDB.BSATN.ITypeRegistrar registrar
    ) =>
        [
            new(
                new(
                    TableName: nameof(TestAutoIncNotInteger),
                    Columns:
                    [
                        new(nameof(AutoIncField), BSATN.AutoIncField.GetAlgebraicType(registrar)),
                        new(nameof(IdentityField), BSATN.IdentityField.GetAlgebraicType(registrar))
                    ],
                    Indexes: [],
                    Constraints: [],
                    Sequences: [],
                    // "system" | "user"
                    TableType: "user",
                    // "public" | "private"
                    TableAccess: "private",
                    Scheduled: null
                ),
                (uint)
                    (
                        (SpacetimeDB.BSATN.AlgebraicType.Ref)new BSATN().GetAlgebraicType(registrar)
                    ).Ref_
            ),
        ];

    static SpacetimeDB.Internal.Filter SpacetimeDB.Internal.ITable<TestAutoIncNotInteger>.CreateFilter() =>
        new(
            [
                new(nameof(AutoIncField), (w, v) => BSATN.AutoIncField.Write(w, (float)v!)),
                new(nameof(IdentityField), (w, v) => BSATN.IdentityField.Write(w, (string)v!))
            ]
        );
} // TestAutoIncNotInteger
