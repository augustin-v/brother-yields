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
Both should return a success message

### utils

Convert felt to usize contract:[0x0638ff764ddd96be61cc35eb6cc7da3702790c4056c3fa976e0931441d33ef1e](https://sepolia.voyager.online/contract/0x0638ff764ddd96be61cc35eb6cc7da3702790c4056c3fa976e0931441d33ef1e#writeContract)