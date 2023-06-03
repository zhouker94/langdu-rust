# langdu-rust

[![Build](https://github.com/OwenZhu/langdu-rust/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/OwenZhu/langdu-rust/actions/workflows/build.yml)

Text To Speech (TTS) command-line tool, powered by Azure Cognitive Services.

## Prerequisites

- Azure [subscription](https://azure.microsoft.com/en-au/free/cognitive-services/).
- Create a Speech resource in the Azure portal.
- Get the Speech resource key and region.

## Quick Start

Add the environment variables

```
export SPEECH_KEY=your-key
export SPEECH_REGION=your-region
```

Run the command to synthesize speech from your `txt` file, and save speech to `mp3`.

```bash
langdu sample.txt output.mp3
```

## Synthesize a dialog

Follow this format in your input `txt` file to synthesize a dialog-style speech with different [voices](https://learn.microsoft.com/en-us/azure/cognitive-services/speech-service/language-support?tabs=tts#prebuilt-neural-voices) with multilingual support.

```
# sample.txt

[en-US-ElizabethNeural] What do you do?
^^^^^^^^^^^^^^^^^^^^^^^
[en-US-JasonNeural] What do I do? System architecture.
^^^^^^^^^^^^^^^^^^^
|
Any supported voice identifier
```
