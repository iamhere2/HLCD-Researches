using System;
using System.Runtime.Serialization;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace ChessApp.ChessApp_Internals.FileStorage_Internals
{
    public class GameSerializer
    {
        private static readonly JsonSerializerSettings JsonOptions =
            new JsonSerializerSettings
            {
                Formatting = Formatting.Indented,
                Converters = new JsonConverter[]
                {
                    new StringEnumConverter(),
                    new CellConverter(),
                    new BoardStateConverter()
                }
            };

        private class StorageModel
        {
            public StorageModel(GameHistory history, Color playerAColor)
            {
                History = history ?? throw new ArgumentNullException(nameof(history));
                PlayerAColor = playerAColor;
            }

            public GameHistory History { get; set; }
            public Color PlayerAColor { get; set; }
        }

        public (GameHistory, Color) Deserialize(string s)
        {
            var model = JsonConvert.DeserializeObject<StorageModel>(s, JsonOptions)
                ?? throw new SerializationException("Can't deserialize game");

            return (model.History, model.PlayerAColor);
        }

        public string Serialize(GameHistory history, Color playerAColor)
            => JsonConvert.SerializeObject(new StorageModel(history, playerAColor), JsonOptions);

        public class CellConverter : JsonConverter<Cell>
        {
            public override Cell ReadJson(JsonReader reader, Type objectType, Cell existingValue, bool hasExistingValue, JsonSerializer serializer)
                => Cell.Parse(reader.ReadAsString() ?? throw new SerializationException("cell string expected"));

            public override void WriteJson(JsonWriter writer, Cell value, JsonSerializer serializer)
                => writer.WriteValue(value.ToString());
        }

        public class BoardStateConverter : JsonConverter<BoardState>
        {
            public override BoardState ReadJson(JsonReader reader, Type objectType, BoardState? existingValue, bool hasExistingValue, JsonSerializer serializer)
            {
                var bs = BoardState.Empty;

                EnsureStartObject(reader);
                while (EnsureRead(reader) != JsonToken.EndObject)
                {
                    var cellStr = EnsureProperty(reader);
                    var cell = Cell.Parse(cellStr);
                    EnsureRead(reader);
                    EnsureStartObject(reader);
                    EnsureRead(reader);
                    EnsureProperty(reader, nameof(Figure));
                    EnsureRead(reader);
                    var figure = serializer.Deserialize<Figure>(reader);
                    EnsureRead(reader);
                    EnsureProperty(reader, nameof(Color));
                    EnsureRead(reader);
                    var color = serializer.Deserialize<Color>(reader);
                    EnsureRead(reader);
                    EnsureEndObject(reader);

                    bs = bs.With(figure, color, cell);
                }

                return bs;
            }

            private string EnsureProperty(JsonReader reader)
            {
                if (reader.TokenType != JsonToken.PropertyName)
                    throw new JsonSerializationException("Expected property name");

                return (reader.Value as string) ?? throw new JsonSerializationException("Expected property name");
            }

            private void EnsureProperty(JsonReader reader, string name)
            {
                if (EnsureProperty(reader) != name)
                    throw new JsonSerializationException($"Expected property {name}");
            }

            private JsonToken EnsureRead(JsonReader reader)
            {
                if (!reader.Read())
                    throw new JsonSerializationException("Unexpected end");

                return reader.TokenType;
            }

            private void EnsureStartObject(JsonReader reader)
            {
                if (reader.TokenType != JsonToken.StartObject)
                    throw new JsonSerializationException("Object expected");
            }

            private void EnsureEndObject(JsonReader reader)
            {
                if (reader.TokenType != JsonToken.EndObject)
                    throw new JsonSerializationException("Object end expected");
            }

            public override void WriteJson(JsonWriter writer, BoardState? value, JsonSerializer serializer)
            {
                if (value is null)
                    throw new ArgumentNullException(nameof(value));

                writer.WriteStartObject();
                foreach (var (cell, (figure, color)) in value.Figures)
                {
                    writer.WritePropertyName(cell.ToString());
                    writer.WriteStartObject();
                    writer.WritePropertyName(nameof(Figure));
                    serializer.Serialize(writer, figure);
                    writer.WritePropertyName(nameof(Color));
                    serializer.Serialize(writer, color);
                    writer.WriteEndObject();
                }
                writer.WriteEndObject();
            }
        }
    }
}
