import fs from "fs";
import processAdjectives from "./processor/adjective";
import processEtc from "./processor/etc";
import processNouns from "./processor/noun";
import processVerbs from "./processor/verb";

export interface AnkiEntry {
  source_word: string;
  target_word: string;
  extra_content?: string;
  tts_side_a?: string;
  tts_side_b?: string;
}

function serializeIntoCsv(entry: AnkiEntry): string {
  return [
    entry.source_word,
    entry.target_word,
    entry.extra_content || "",
    entry.tts_side_a ? `[sound:${entry.tts_side_a}.mp3]` : "",
    entry.tts_side_b ? `[sound:${entry.tts_side_b}.mp3]` : "",
  ]
    .map((line) => line.replace(/\n/g, "<br>"))
    .map((line) => line.replace(/"/g, '""'))
    .map((line) => `"${line}"`)
    .join(";");
}

async function* iterateAllEntries(): AsyncGenerator<AnkiEntry> {
  yield* processVerbs;
  yield* processNouns;
  yield* processAdjectives;
  yield* processEtc;
}

const output_stream = fs.createWriteStream("deck.csv", {
  encoding: "utf-8",
  flags: "w",
});

for await (const entry of iterateAllEntries()) {
  output_stream.write(serializeIntoCsv(entry), "utf-8");
  output_stream.write("\n", "utf-8");
}
