# brother-yields

Project for the Starknet winter hackathon 2025!

## Quickstart
### First terminal
```bash
cp .env.example .env
```
configure .env then
```bash
cargo run
```
### Second terminal
```bash
npm i && npm run dev
```
---
for test prompt open second terminal to test with 
```bash
curl -X POST http://localhost:5050/launch
```
or to actually POST a prompt with:
```bash
curl -X POST http://localhost:5050/prompt \
  -H "Content-Type: application/json" \
  -d '{"prompt": "YOUR_PROMPT_HERE"}'
```
Both should return a success message with the returned prompt inside

### contract:
[0x03a4a729f942c231a9c95a25b5d9624fb1ae93e9db7ec98449e1ddff12437f38](https://sepolia.voyager.online/contract/0x03a4a729f942c231a9c95a25b5d9624fb1ae93e9db7ec98449e1ddff12437f38)