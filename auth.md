---
title: "SpaceCorps Universal Authentication Standard"
description: "Authentication flows, OS keystore integration, headless CI/CD piping, and multi-account security across all SpaceCorps CLI tools."
author: "SpaceCorps"
date: "2026-09-24"
---

# SpaceCorps Universal Authentication Standard

All tools in the SpaceCorps CLI fleet share a unified credential architecture designed for security, developer ergonomics, and autonomous agent orchestration.

## Core Authentication Principles

1. **Native OS Keystores:** By default, credentials (API tokens, passwords, secrets) are stored in your platform's native keyring:
   - **macOS:** macOS Keychain Services (`security add-generic-password`)
   - **Linux:** freedesktop.org Secret Service daemon (`secret-tool`)
   - **Windows:** Windows DPAPI (`CryptProtectData`)
2. **Headless Stdin Piping:** In automated CI/CD runners, containerized agent runners, or scripts, pass tokens without shell history leakage:
   ```bash
   echo "$API_TOKEN" | <binary> login [account_alias] --api-key-stdin
   ```
3. **Multi-Account Scoping:** Manage multiple staging, production, or customer accounts safely using `--account <name>` (short `-a <name>`):
   ```bash
   sliplane services list --account staging
   sliplane services list --account prod
   ```
4. **Environment Variable Fallback:** Each CLI supports an environment variable fallback (e.g. `SLIPLANE_API_KEY`, `EXA_API_KEY`, `FIRECRAWL_API_KEY`, `APIFY_TOKEN`, `CLOUDFLARE_API_TOKEN`) for zero-configuration transient execution.

## Standard Exit Codes for Auth

- `3` (`Unauthorized / auth_required`): Credentials missing or rejected by upstream service.
- `2` (`Bad Request / invalid_input`): Malformed token syntax or missing required argument.
