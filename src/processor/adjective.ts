import { AnkiEntry } from "../main";
import { synthehise } from "../tts";
import { capitalize, processor } from ".";

async function processAdjective(line: string): Promise<AnkiEntry> {
  const [source_word, target_word] = line.split(",").map((elem) => elem.trim());

  return {
    source_word: source_word,
    target_word: target_word,
    tts_side_a: await synthehise(source_word, `${capitalize(source_word)}.`),
  };
}

export default processor("words/adjectives.csv", processAdjective);
