import { JSDOM } from "jsdom";
import fs from "fs";
import chalk from "chalk";

const CONJUGATION_ENDPOINT_STUB = "https://conjugator.reverso.net/conjugation";
const CONJUGATION_CACHE_DIR = `.cache/conjugations`;

export type Conjugation = {
  kind: "regular" | "irregular";
  person: string;
  root: string;
  postfix: string;
};

export interface Tense {
  s1: Conjugation;
  s2: Conjugation;
  s3: Conjugation;
  p1: Conjugation;
  p2: Conjugation;
  p3: Conjugation;
}

function extractConjugation(li: HTMLLIElement): Conjugation {
  const person = li.querySelector("i.graytxt")?.innerHTML;
  if (person === undefined) {
    throw new Error("Conjugation person cannot be null");
  }

  // Roots
  const root = li.querySelector("i.verbtxt")?.innerHTML || "";
  const regular_postfix = li.querySelector("i.verbtxt-term")?.innerHTML;
  const irregular_postfix = li.querySelector("i.verbtxt-term-irr")?.innerHTML;

  if (!irregular_postfix) {
    if (!regular_postfix) {
      throw Error(`No regular postfix found for conjugation '${person}'`);
    } else {
      return {
        kind: "regular",
        person: person,
        root: root,
        postfix: regular_postfix,
      };
    }
  } else {
    return {
      kind: "irregular",
      person: person,
      root: root,
      postfix: irregular_postfix,
    };
  }
}

function findConjugation(
  person: string,
  conjugations: Conjugation[]
): Conjugation {
  const conjugation = conjugations.find((conj) => conj.person === person);
  if (!conjugation) {
    throw new Error(`Cannot find conjugation '${person}'`);
  } else {
    return conjugation;
  }
}

function extractTense(box: HTMLDivElement): Tense {
  const conjugation_elements: HTMLLIElement[] = Array.from(
    box.querySelectorAll("ul.wrap-verbs-listing > li")
  );
  const conjugations = conjugation_elements.map((li) => extractConjugation(li));

  return {
    s1: findConjugation("eu", conjugations),
    s2: findConjugation("tu", conjugations),
    s3: findConjugation("ele/ela/você", conjugations),
    p1: findConjugation("nós", conjugations),
    p2: findConjugation("vós", conjugations),
    p3: findConjugation("eles/elas/vocês", conjugations),
  };
}

export async function conjugate(word: string, tense: string): Promise<Tense> {
  const conjugation_cache_file = `${CONJUGATION_CACHE_DIR}/${word}.json`;
  if (fs.existsSync(conjugation_cache_file)) {
    console.log(chalk.green(`  Conjugation: cached`));
    return JSON.parse(
      fs.readFileSync(conjugation_cache_file, { encoding: "utf-8" })
    );
  }

  console.log(chalk.yellow("  Conjugation: fetched"));
  const word_endpoint = `${CONJUGATION_ENDPOINT_STUB}-portuguese-verb-${word}.html`;

  const response = await fetch(word_endpoint);
  const response_text = await response.text();

  const dom = new JSDOM(response_text);
  const tense_boxes: HTMLDivElement[] = Array.from(
    dom.window.document.querySelectorAll(
      "div.result-block-api > div.word-wrap-row > div.wrap-three-col > div.blue-box-wrap"
    )
  );

  const selected_tense = tense_boxes.filter(
    (box) => box.querySelector("p")?.innerHTML === tense
  );

  const extracted_tense = extractTense(selected_tense[0]);
  fs.writeFileSync(
    conjugation_cache_file,
    JSON.stringify(extracted_tense, null, 2)
  );
  return extracted_tense;
}

fs.mkdirSync(CONJUGATION_CACHE_DIR, { recursive: true });
