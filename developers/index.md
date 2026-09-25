# SpaceCorps Developer Portal

> API keys, keystore security, quickstart guides, OpenAPI 3.1 specifications, and interactive sandbox environments.

## 1. Machine-Readable API Specifications

- **OpenAPI 3.1 JSON:** https://spacecorps.github.io/openapi.json
- **OpenAPI 3.1 YAML:** https://spacecorps.github.io/openapi.yaml
- **MCP Server Manifest:** https://spacecorps.github.io/server.json
- **MCP Server Card:** https://spacecorps.github.io/.well-known/mcp

## 2. Authentication & Keystore Security

SpaceCorps CLI binaries integrate with operating system secure keystores:

```bash
# Interactive login
sliplane login default

# Non-interactive stdin login for CI/CD runners:
echo "$API_KEY" | sliplane login default --api-key-stdin
```

## 3. Sandbox & Local Test Invocation

```bash
# Molecular simulation sandbox
spacemd simulate --input examples/argon_liquid.json --steps 1000 --json

# Thermal electron emission calculation
spaceemit compute --field 5e9 --work-function 4.5 --temperature 300 --json
```

## 4. Agent Guidance

- [Curated llms.txt](https://spacecorps.github.io/llms.txt)
- [Comprehensive Manual (llms-full.txt)](https://spacecorps.github.io/llms-full.txt)
- [When to Use Decision Guide](https://spacecorps.github.io/.well-known/agent-instructions.md)
