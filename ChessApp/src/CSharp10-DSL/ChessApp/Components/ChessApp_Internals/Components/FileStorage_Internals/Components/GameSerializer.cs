using ChessApp.Components.ChessApp_Internals.Components.FileStorage_Internals.Components;
using HLCD.ChessAppExampleWithDSL.ChessApp_Internals.FileStorage_Internals.Data;

namespace HLCD.ChessAppExampleWithDSL.Components.ChessApp_Internals.Components.FileStorage_Internals.Components
{
    [Component("CA-FS-GS")]
    public sealed class GameSerializer
    {
        private static readonly JsonSerializerSettings JsonOptions =
            new()
            {
                Formatting = Formatting.Indented,
                ReferenceLoopHandling = ReferenceLoopHandling.Ignore,
                Converters = new JsonConverter[]
                {
                    new StringEnumConverter(),
                    new CellConverter(),
                    new BoardStateConverter(),
                    new TurnConverter()
                }
            };


        public (GameHistory, Color) Deserialize(string s)
        {
            var model = JsonConvert.DeserializeObject<StorageModel>(s, JsonOptions)
                ?? throw new SerializationException("Can't deserialize game");

            return (model.History, model.PlayerAColor);
        }

        public string Serialize(GameHistory history, Color playerAColor)
            => JsonConvert.SerializeObject(new StorageModel(history, playerAColor), JsonOptions);

        private sealed class CellConverter : JsonConverter<Cell>
        {
            public override Cell ReadJson(JsonReader reader, Type objectType, Cell existingValue, bool hasExistingValue, JsonSerializer serializer)
            {
                if (reader is null)
                    throw new ArgumentNullException(nameof(reader));

                return Cell.Parse(reader.ReadAsString() ?? throw new SerializationException("cell string expected"));
            }

            public override void WriteJson(JsonWriter writer, Cell value, JsonSerializer serializer)
            {
                if (writer is null)
                    throw new ArgumentNullException(nameof(writer));

                writer.WriteValue(value.ToString());
            }
        }

        private sealed class TurnConverter : JsonConverter<Turn>
        {
            public override Turn ReadJson(JsonReader reader, Type objectType, Turn? existingValue, bool hasExistingValue, JsonSerializer serializer)
            {
                if (reader is null)
                    throw new ArgumentNullException(nameof(reader));

                if (serializer is null)
                    throw new ArgumentNullException(nameof(serializer));

                reader.EnsureStartObject();
                reader.EnsureRead();
                reader.EnsureProperty(nameof(Move.From));
                var from = serializer.Deserialize<Cell>(reader);
                reader.EnsureRead();
                reader.EnsureProperty(nameof(Move.To));
                var to = serializer.Deserialize<Cell>(reader);
                reader.EnsureRead();
                reader.EnsureEndObject();

                return new Move(from, to);
            }

            public override void WriteJson(JsonWriter writer, Turn? value, JsonSerializer serializer)
            {
                if (writer is null)
                    throw new ArgumentNullException(nameof(writer));

                if (value is null)
                    throw new ArgumentNullException(nameof(value));

                if (serializer is null)
                    throw new ArgumentNullException(nameof(serializer));

                if (value is Move move)
                {
                    writer.WriteStartObject();
                    writer.WritePropertyName(nameof(Move.From));
                    serializer.Serialize(writer, move.From);
                    writer.WritePropertyName(nameof(Move.To));
                    serializer.Serialize(writer, move.To);
                    writer.WriteEndObject();
                }
                else
                {
                    throw new JsonSerializationException("Unsupported Turn kind");
                }
            }
        }

        private sealed class BoardStateConverter : JsonConverter<BoardState>
        {
            public override BoardState ReadJson(JsonReader reader, Type objectType, BoardState? existingValue, bool hasExistingValue, JsonSerializer serializer)
            {
                if (reader is null)
                    throw new ArgumentNullException(nameof(reader));

                if (serializer is null)
                    throw new ArgumentNullException(nameof(serializer));

                var bs = BoardState.Empty;

                reader.EnsureStartObject();
                while (reader.EnsureRead() != JsonToken.EndObject)
                {
                    var cellStr = reader.EnsureProperty();
                    var cell = Cell.Parse(cellStr);
                    reader.EnsureRead();
                    reader.EnsureStartObject();
                    reader.EnsureRead();
                    reader.EnsureProperty(nameof(Figure));
                    reader.EnsureRead();
                    var figure = serializer.Deserialize<Figure>(reader);
                    reader.EnsureRead();
                    reader.EnsureProperty(nameof(Color));
                    reader.EnsureRead();
                    var color = serializer.Deserialize<Color>(reader);
                    reader.EnsureRead();
                    reader.EnsureEndObject();

                    bs = bs.With(figure, color, cell);
                }

                return bs;
            }

            public override void WriteJson(JsonWriter writer, BoardState? value, JsonSerializer serializer)
            {
                if (writer is null)
                    throw new ArgumentNullException(nameof(writer));

                if (value is null)
                    throw new ArgumentNullException(nameof(value));

                if (serializer is null)
                    throw new ArgumentNullException(nameof(serializer));

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
