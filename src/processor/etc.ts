import { processor, splitAndClean } from ".";
import { AnkiEntry } from "../main";

async function process(line: string): Promise<AnkiEntry> {
  const [source_word, target_word] = splitAndClean(line);

  return {
    source_word: source_word,
    target_word: target_word,
  };
}

export default processor("words/etc.csv", process);
