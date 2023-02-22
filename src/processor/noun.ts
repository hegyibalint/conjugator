import { capitalize, processor, splitAndClean } from ".";
import { AnkiEntry } from "../main";
import { synthehise } from "../tts";

type GrammaticalGender = "f" | "m";

const GR_GENDER_TO_ARTICLE: Record<GrammaticalGender, string> = {
  f: "O",
  m: "A",
};

async function processNoun(line: string): Promise<AnkiEntry> {
  const [source_word, source_gender, target_word] = splitAndClean(line);

  const article = GR_GENDER_TO_ARTICLE[source_gender as GrammaticalGender];
  const articled_source_word = `${article} ${source_word}`;

  return {
    source_word: articled_source_word,
    target_word: target_word,
    tts_side_a: await synthehise(source_word, `${capitalize(source_word)}.`),
  };
}

export default processor("words/nouns.csv", processNoun);
