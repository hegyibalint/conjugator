import fs from "fs";
import readline from "readline";
import { capitalize, processor, splitAndClean } from ".";
import { conjugate, Conjugation, Tense } from "../conjugator";
import { AnkiEntry } from "../main";
import { synthehise as synthesize } from "../tts";

function formatAsPlaintext(conjugation: Conjugation) {
  switch (conjugation.kind) {
    case "regular":
      return conjugation.root + conjugation.postfix;
    case "irregular":
      return conjugation.verb;
  }
}

function formatAsHTML(conjugation: Conjugation) {
  switch (conjugation.kind) {
    case "regular":
      return `${conjugation.root}<span style="color: green;">${conjugation.postfix}</span>`;
    case "irregular":
      return `<span style="color: red;">${conjugation.verb}</span>`;
  }
}

async function createWordTTSSample(word: string): Promise<string> {
  const word_id = `${word.replace(" ", "-")}+word`;
  return synthesize(word_id, `${capitalize(word)}.`);
}

async function createConjugationTTSSample(
  word: string,
  tense: Tense
): Promise<string> {
  const word_id = `${word.replace(" ", "-")}+conj`;

  return synthesize(
    word_id,
    [
      `Eu ${formatAsPlaintext(tense.s1)}.`,
      `Tu ${formatAsPlaintext(tense.s2)}.`,
      `Ele ${formatAsPlaintext(tense.s3)}.`,
      `Nós ${formatAsPlaintext(tense.p1)}.`,
      `Vós ${formatAsPlaintext(tense.p2)}.`,
      `Eles ${formatAsPlaintext(tense.p3)}.`,
    ].join(" ")
  );
}

function createConjugationDescription(tense: Tense): string {
  return [
    `Eu ${formatAsHTML(tense.s1)}.`,
    `Tu ${formatAsHTML(tense.s2)}.`,
    `Ele ${formatAsHTML(tense.s3)}.`,
    `Nós ${formatAsHTML(tense.p1)}.`,
    `Vós ${formatAsHTML(tense.p2)}.`,
    `Eles ${formatAsHTML(tense.p3)}.`,
  ].join("\n");
}

async function processVerb(line: string): Promise<AnkiEntry> {
  const [source_word, target_word] = splitAndClean(line);
  let conjugation = await conjugate(source_word, "Presente");

  return {
    source_word: source_word,
    target_word: target_word,
    extra_content: createConjugationDescription(conjugation),
    tts_side_a: await createWordTTSSample(source_word),
    tts_side_b: await createConjugationTTSSample(source_word, conjugation),
  };
}

export default processor("words/verbs.csv", processVerb);
