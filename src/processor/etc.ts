import { capitalize, createWordID, processor, splitAndClean } from ".";
import { AnkiEntry } from "../main";
import { synthesise } from "../tts";

async function process(line: string): Promise<AnkiEntry> {
  const [source_word, target_word] = splitAndClean(line);

  const word_id = createWordID(source_word);
  const tts_sample_text =
    capitalize(source_word) + (source_word.endsWith("?") ? "" : ".");

  return {
    source_word: source_word,
    target_word: target_word,
    tts_side_a: await synthesise(word_id, tts_sample_text),
  };
}

export default processor("words/etc.csv", process);
