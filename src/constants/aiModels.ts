export const AI_PROVIDERS = [
  { value: "ollama", label: "Ollama (Lokal)" },
  { value: "gemini", label: "Google Gemini" },
  { value: "openai", label: "OpenAI / Compatible API" },
] as const;

export const GEMINI_MODELS = [
  "gemini-3.5-flash",
  "gemini-2.5-flash",
  "gemini-3.5-flash-lite",
  "gemini-3.5-flash-lite",
] as const;

export const OPENAI_MODELS = [
  "gpt-4o-mini",
  "gpt-4o",
  "gpt-4.1-mini",
  "gpt-4.1",
  "o3-mini",
] as const;

export const OLLAMA_MODELS = [
  "llama3",
  "llama3.1",
  "llama3.2",
  "mistral",
  "gemma2",
  "qwen2.5",
  "phi3",
] as const;

export const WHISPER_MODELS = [
  { value: "tiny", label: "Tiny (Cepat)" },
  { value: "base", label: "Base" },
  { value: "small", label: "Small (Rekomendasi)" },
  { value: "medium", label: "Medium" },
  { value: "large-v3", label: "Large v3" },
  { value: "large-v3-turbo", label: "Large v3 Turbo" },
] as const;
