﻿//HintName: TestTypeParams_T_.cs
// <auto-generated />
#nullable enable

partial struct TestTypeParams<T> : SpacetimeDB.BSATN.IStructuralReadWrite
{
    public void ReadFields(System.IO.BinaryReader reader)
    {
        Field = BSATN.Field.Read(reader);
    }

    public void WriteFields(System.IO.BinaryWriter writer)
    {
        BSATN.Field.Write(writer, Field);
    }

    public readonly partial struct BSATN : SpacetimeDB.BSATN.IReadWrite<TestTypeParams<T>>
    {
        internal static readonly TRW Field = new();

        public TestTypeParams<T> Read(System.IO.BinaryReader reader) =>
            SpacetimeDB.BSATN.IStructuralReadWrite.Read<TestTypeParams<T>>(reader);

        public void Write(System.IO.BinaryWriter writer, TestTypeParams<T> value)
        {
            value.WriteFields(writer);
        }

        public SpacetimeDB.BSATN.AlgebraicType GetAlgebraicType(
            SpacetimeDB.BSATN.ITypeRegistrar registrar
        ) =>
            registrar.RegisterType<TestTypeParams<T>>(
                _ => new SpacetimeDB.BSATN.AlgebraicType.Product(
                    new SpacetimeDB.BSATN.AggregateElement[]
                    {
                        new(nameof(Field), Field.GetAlgebraicType(registrar))
                    }
                )
            );
    }
} // TestTypeParams<T>
