using Newtonsoft.Json;

namespace ChessApp.Components.ChessApp_Internals.Components.FileStorage_Internals.Components
{
    static class JsonReaderExtensions
    {
        public static string EnsureProperty(this JsonReader reader)
        {
            if (reader.TokenType != JsonToken.PropertyName)
                throw new JsonSerializationException("Expected property name");

            return reader.Value as string ?? throw new JsonSerializationException("Expected property name");
        }

        public static void EnsureProperty(this JsonReader reader, string name)
        {
            if (EnsureProperty(reader) != name)
                throw new JsonSerializationException($"Expected property {name}");
        }

        public static JsonToken EnsureRead(this JsonReader reader)
        {
            if (!reader.Read())
                throw new JsonSerializationException("Unexpected end");

            return reader.TokenType;
        }

        public static void EnsureStartObject(this JsonReader reader)
        {
            if (reader.TokenType != JsonToken.StartObject)
                throw new JsonSerializationException("Object expected");
        }

        public static void EnsureEndObject(this JsonReader reader)
        {
            if (reader.TokenType != JsonToken.EndObject)
                throw new JsonSerializationException("Object end expected");
        }
    }
}
