import { AnkiEntry } from "../main";
import { synthesise } from "../tts";
import { capitalize, createWordID, processor } from ".";

async function processAdjective(line: string): Promise<AnkiEntry> {
  const [source_word, target_word] = line.split(",").map((elem) => elem.trim());
  const word_id = createWordID(source_word);

  return {
    source_word: source_word,
    target_word: target_word,
    tts_side_a: await synthesise(word_id, `${capitalize(source_word)}.`),
  };
}

export default processor("words/adjectives.csv", processAdjective);
