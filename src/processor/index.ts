import { AnkiEntry } from "../main";
import readline from "readline";
import fs from "fs";

export type Processor = (line: string) => Promise<AnkiEntry>;

export function createWordID(word: string, type?: string): string {
  const word_segments = word.match(/\w*/g);
  if (!word_segments) {
    throw new Error("Word cannot be sanitized");
  }

  const sanitized_word = word_segments.filter((m) => m.length > 0).join("-");
  if (!type) {
    return sanitized_word;
  } else {
    return `${sanitized_word}+${type}`;
  }
}

export function capitalize(word: string) {
  return word.charAt(0).toUpperCase() + word.substring(1);
}

export function splitAndClean(line: string): string[] {
  return line.split(",").map((line) => line.trim());
}

export async function* processor(
  file: string,
  processor: Processor
): AsyncGenerator<AnkiEntry> {
  const rl = readline.createInterface(fs.createReadStream(file));

  for await (const line of rl) {
    if (!line.startsWith("#") && line.length > 0) {
      console.log(`Processing ${splitAndClean(line)[0]}`);
      yield processor(line);
    }
  }
}
