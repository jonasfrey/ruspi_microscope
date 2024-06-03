curl https://api.openai.com/v1/audio/speech \
  -H "Authorization: Bearer sk-proj-97TkvF5KzMtMLaPFLooVT3BlbkFJQ0WFgNyqxy95g3VlJiNH" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "tts-1",
    "input": "The following is IPA and if spelled correctly it sounds like '/mʊmˈbaɪ/'",
    "voice": "alloy"
  }' \
  --output test2.mp3