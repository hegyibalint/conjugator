import { AnkiEntry } from "../main";
import readline from "readline";
import fs from "fs";

export type Processor = (line: string) => Promise<AnkiEntry>;

export async function* processor(
  file: string,
  processor: Processor
): AsyncGenerator<AnkiEntry> {
  const rl = readline.createInterface(fs.createReadStream(file));

  for await (const line of rl) {
    console.log(`Processing ${splitAndClean(line)[0]}`);
    yield processor(line);
  }
}

export function capitalize(word: string) {
  return word.charAt(0).toUpperCase() + word.substring(1);
}

export function splitAndClean(line: string): string[] {
  return line.split(",").map((line) => line.trim());
}
