import { JSDOM } from "jsdom";
import fs from "fs";
import chalk from "chalk";

const CONJUGATION_ENDPOINT_STUB = "https://conjugator.reverso.net/conjugation";
const CONJUGATION_CACHE_DIR = `.cache/conjugations`;

export type RegularConjugation = {
  kind: "regular";
  person: string;
  root: string;
  postfix: string;
};

export type IrregularConjugation = {
  kind: "irregular";
  person: string;
  verb: string;
};

export type Conjugation = RegularConjugation | IrregularConjugation;

export interface Tense {
  s1: Conjugation;
  s2: Conjugation;
  s3: Conjugation;
  p1: Conjugation;
  p2: Conjugation;
  p3: Conjugation;
}

function extractConjugation(li: HTMLLIElement): Conjugation {
  const person_element = li.querySelector("i.graytxt");

  if (person_element === null) {
    throw new Error("Conjugation person cannot be null");
  }

  const irregular_term = li.querySelector("i.verbtxt-term-irr");
  if (irregular_term !== null) {
    return {
      kind: "irregular",
      person: person_element.innerHTML,
      verb: irregular_term.innerHTML,
    };
  }

  const regular_term_root = li.querySelector("i.verbtxt");
  const regular_term_postfix = li.querySelector("i.verbtxt-term");
  if (regular_term_root === null || regular_term_postfix === null) {
    throw new Error("Cannot find conjugated verb");
  }

  return {
    kind: "regular",
    person: person_element.innerHTML,
    root: regular_term_root.innerHTML,
    postfix: regular_term_postfix.innerHTML,
  };
}

function extractTense(box: HTMLDivElement): Tense {
  const conjugation_elements: HTMLLIElement[] = Array.from(
    box.querySelectorAll("ul.wrap-verbs-listing > li")
  );
  const conjugations = conjugation_elements.map((li) => extractConjugation(li));

  return {
    s1: conjugations.find((c) => c.person === "eu")!,
    s2: conjugations.find((c) => c.person === "tu")!,
    s3: conjugations.find((c) => c.person === "ele/ela/você")!,
    p1: conjugations.find((c) => c.person === "nós")!,
    p2: conjugations.find((c) => c.person === "vós")!,
    p3: conjugations.find((c) => c.person === "eles/elas/vocês")!,
  };
}

export async function conjugate(word: string, tense: string) {
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
