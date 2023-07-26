import fs from "fs";

import { Readable } from "stream";
import {
  PollyClient,
  SynthesizeSpeechCommand,
  SynthesizeSpeechCommandInput,
} from "@aws-sdk/client-polly";
import chalk from "chalk";

const TTS_CACHE_DIR = `.cache/tts`;

const client = new PollyClient({});

export async function synthesise(
  word_id: string,
  text: string
): Promise<string> {
  const word_tts_path = `${TTS_CACHE_DIR}/${word_id}.mp3`;

  if (!fs.existsSync(word_tts_path)) {
    console.log(chalk.yellow(`  TTS: requested for ${word_id}`));
    const input: SynthesizeSpeechCommandInput = {
      Text: text,
      LanguageCode: "pt-PT",
      VoiceId: "Ines",
      Engine: "neural",
      OutputFormat: "mp3",
    };
    const command = new SynthesizeSpeechCommand(input);
    const response = await client.send(command);
    const input_stream = response.AudioStream as Readable;

    await input_stream.pipe(fs.createWriteStream(word_tts_path));
  } else {
    console.log(chalk.green(`  TTS: cached for ${word_id}`));
  }

  return word_id;
}

fs.mkdirSync(TTS_CACHE_DIR, { recursive: true });
