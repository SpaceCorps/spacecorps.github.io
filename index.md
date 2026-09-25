---
title: "SpaceCorps Developer Portal, Systems & Simulation Registry"
description: "High-performance native Rust systems: 23 autonomous AI agent CLIs, the cross-platform Space3d game engine, and Space3d-Molecular scientific computing (atomistic molecular dynamics and electron-emission physics)."
author: "SpaceCorps"
date: "2026-09-25"
canonical: "https://spacecorps.github.io/index.md"
---

# SpaceCorps Developer Portal & Systems Registry

SpaceCorps engineers high-performance native Rust systems, cross-platform 3D graphics engines, atomistic physics simulations, and autonomous agent infrastructure. Every CLI and engine in our portfolio is built with pure Rust 2024, featuring sub-3ms cold starts, GPU-driven indirect drawing, rigorous physics validation (74M–108M atom-steps/s in molecular dynamics, 14 µs emission solves), and structured machine-readable outputs.

## Flagship Systems & The SpaceCorps Fleet

| Tool / Engine             | Primary Command / Crate | Category             | Docs                                                        | GitHub                                                        | Agent Manual                                                            |
| ------------------------- | ----------------------- | -------------------- | ----------------------------------------------------------- | ------------------------------------------------------------- | ----------------------------------------------------------------------- |
| **Space3d-Molecular**     | `spacemd` / `spaceemit` | Simulation & Physics | [Overview](https://github.com/SpaceCorps/Space3d-Molecular) | [GitHub](https://github.com/SpaceCorps/Space3d-Molecular)     | [llms.txt](https://spacecorps.github.io/llms.txt)                       |
| **Space3d Engine**        | `space3d`               | 3D Graphics & Games  | [Showcase](https://spacecorps.github.io/#space3d)           | [SpaceCorps](https://github.com/SpaceCorps)                   | [llms.txt](https://spacecorps.github.io/llms.txt)                       |
| **Sliplane CLI**          | `sliplane`              | Cloud & Hosting      | [Docs](https://spacecorps.github.io/Sliplane-Cli/)          | [GitHub](https://github.com/SpaceCorps/Sliplane-Cli)          | [llms.txt](https://spacecorps.github.io/Sliplane-Cli/llms.txt)          |
| **Cloudflare CLI**        | `cloudflare`            | Cloud & Hosting      | [Docs](https://spacecorps.github.io/Cloudflare-Cli/)        | [GitHub](https://github.com/SpaceCorps/Cloudflare-Cli)        | [llms.txt](https://spacecorps.github.io/Cloudflare-Cli/llms.txt)        |
| **Storage CLI**           | `storage`               | Cloud & Hosting      | [Docs](https://spacecorps.github.io/Storage-Cli/)           | [GitHub](https://github.com/SpaceCorps/Storage-Cli)           | [llms.txt](https://spacecorps.github.io/Storage-Cli/llms.txt)           |
| **Exa CLI**               | `exa`                   | Search & Scraping    | [Docs](https://spacecorps.github.io/Exa-Cli/)               | [GitHub](https://github.com/SpaceCorps/Exa-Cli)               | [llms.txt](https://spacecorps.github.io/Exa-Cli/llms.txt)               |
| **Firecrawl CLI**         | `firecrawl`             | Search & Scraping    | [Docs](https://spacecorps.github.io/Firecrawl-Cli/)         | [GitHub](https://github.com/SpaceCorps/Firecrawl-Cli)         | [llms.txt](https://spacecorps.github.io/Firecrawl-Cli/llms.txt)         |
| **TIC CLI**               | `tic`                   | Search & Scraping    | [Docs](https://spacecorps.github.io/Tic-Cli/)               | [GitHub](https://github.com/SpaceCorps/Tic-Cli)               | [llms.txt](https://spacecorps.github.io/Tic-Cli/llms.txt)               |
| **Spaceship CLI**         | `spaceship`             | Domains & DNS        | [Docs](https://spacecorps.github.io/Spaceship-Cli/)         | [GitHub](https://github.com/SpaceCorps/Spaceship-Cli)         | [llms.txt](https://spacecorps.github.io/Spaceship-Cli/llms.txt)         |
| **Loopia CLI**            | `loopia`                | Domains & DNS        | [Docs](https://spacecorps.github.io/Loopia-Cli/)            | [GitHub](https://github.com/SpaceCorps/Loopia-Cli)            | [llms.txt](https://spacecorps.github.io/Loopia-Cli/llms.txt)            |
| **Namecheap CLI**         | `namecheap-cli`         | Domains & DNS        | [Repo](https://github.com/SpaceCorps/namecheap-cli)         | [GitHub](https://github.com/SpaceCorps/namecheap-cli)         | —                                                                       |
| **Reddit CLI**            | `reddit`                | Social Intelligence  | [Docs](https://spacecorps.github.io/Reddit-Cli/)            | [GitHub](https://github.com/SpaceCorps/Reddit-Cli)            | [llms.txt](https://spacecorps.github.io/Reddit-Cli/llms.txt)            |
| **Twitter CLI**           | `twitter`               | Social Intelligence  | [Docs](https://spacecorps.github.io/Twitter-Cli/)           | [GitHub](https://github.com/SpaceCorps/Twitter-Cli)           | [llms.txt](https://spacecorps.github.io/Twitter-Cli/llms.txt)           |
| **LinkedIn CLI**          | `linkedin`              | Social Intelligence  | [Docs](https://spacecorps.github.io/Linkedin-Cli/)          | [GitHub](https://github.com/SpaceCorps/Linkedin-Cli)          | [llms.txt](https://spacecorps.github.io/Linkedin-Cli/llms.txt)          |
| **YouTube CLI**           | `youtube`               | Social Intelligence  | [Docs](https://spacecorps.github.io/Youtube-Cli/)           | [GitHub](https://github.com/SpaceCorps/Youtube-Cli)           | [llms.txt](https://spacecorps.github.io/Youtube-Cli/llms.txt)           |
| **TikTok CLI**            | `tiktok`                | Social Intelligence  | [Docs](https://spacecorps.github.io/TikTok-Cli/)            | [GitHub](https://github.com/SpaceCorps/TikTok-Cli)            | [llms.txt](https://spacecorps.github.io/TikTok-Cli/llms.txt)            |
| **Product Hunt CLI**      | `producthunt`           | Social Intelligence  | [Docs](https://spacecorps.github.io/Producthunt-Cli/)       | [GitHub](https://github.com/SpaceCorps/Producthunt-Cli)       | [llms.txt](https://spacecorps.github.io/Producthunt-Cli/llms.txt)       |
| **Stack Overflow CLI**    | `stackoverflow`         | Social Intelligence  | [Docs](https://spacecorps.github.io/Stackoverflow-Cli/)     | [GitHub](https://github.com/SpaceCorps/Stackoverflow-Cli)     | [llms.txt](https://spacecorps.github.io/Stackoverflow-Cli/llms.txt)     |
| **Bugsink CLI**           | `bugsink`               | DevOps & Alerting    | [Docs](https://spacecorps.github.io/Bugsink-Cli/)           | [GitHub](https://github.com/SpaceCorps/Bugsink-Cli)           | [llms.txt](https://spacecorps.github.io/Bugsink-Cli/llms.txt)           |
| **HeyReach CLI**          | `heyreach`              | DevOps & Alerting    | [Docs](https://spacecorps.github.io/Heyreach-Cli/)          | [GitHub](https://github.com/SpaceCorps/Heyreach-Cli)          | [llms.txt](https://spacecorps.github.io/Heyreach-Cli/llms.txt)          |
| **Gmail CLI**             | `gmail`                 | DevOps & Alerting    | [Docs](https://spacecorps.github.io/Gmail-Cli/)             | [GitHub](https://github.com/SpaceCorps/Gmail-Cli)             | [llms.txt](https://spacecorps.github.io/Gmail-Cli/llms.txt)             |
| **Notify CLI**            | `notify`                | DevOps & Alerting    | [Docs](https://spacecorps.github.io/Notify-Cli/)            | [GitHub](https://github.com/SpaceCorps/Notify-Cli)            | [llms.txt](https://spacecorps.github.io/Notify-Cli/llms.txt)            |
| **GitHub Issue Importer** | `gh-issue-import`       | DevOps & Alerting    | [Docs](https://spacecorps.github.io/Github-Issue-Importer/) | [GitHub](https://github.com/SpaceCorps/Github-Issue-Importer) | [llms.txt](https://spacecorps.github.io/Github-Issue-Importer/llms.txt) |
| **Samsung ArtMode CLI**   | `samsung-artmode`       | Hardware & IoT       | [Docs](https://spacecorps.github.io/Samsung-Artmode-Cli/)   | [GitHub](https://github.com/SpaceCorps/Samsung-Artmode-Cli)   | [llms.txt](https://spacecorps.github.io/Samsung-Artmode-Cli/llms.txt)   |
| **OpenAppleModels**       | `oam`                   | AI & Game Engines    | [Docs](https://spacecorps.github.io/open-apple-models/)     | [GitHub](https://github.com/SpaceCorps/open-apple-models)     | [llms.txt](https://spacecorps.github.io/open-apple-models/llms.txt)     |

## Universal Engineering Standards

1. **Pure Rust 2024**: Standalone static binaries and crates compiled with fat LTO, strict optimization, and zero C/C++/Fortran runtime dependencies.
2. **Deterministic Agent Protocol**: Human-friendly YAML on `stdout` by default; strict JSON with `--json` for LLM tool calling.
3. **Scientific Numerical Validation**: Validated against published benchmarks (Lennard-Jones EOS, Ewald Madelung 1.74756459, liquid argon self-diffusion 2.43×10⁻⁵ cm²/s, Murphy-Good & Richardson-Schottky limits).
4. **OS Keystore Encryption**: Secure credential storage using macOS Keychain, Linux Secret Service, and Windows DPAPI.
5. **Custom Cargo Sparse Registry**: Dedicated package registry hosted at `https://spacecorps-registry.sliplane.app` for high-speed crate and binary distribution.

## Resources & Documentation

- [Organization Overview](https://spacecorps.github.io/about.html)
- [Space3d-Molecular GitHub](https://github.com/SpaceCorps/Space3d-Molecular)
- [Package Registry](https://spacecorps-registry.sliplane.app)
- [OpenAPI 3.1 Spec](https://github.com/SpaceCorps/Space3d-Molecular/blob/main/docs/openapi.yaml)
- [Contact & Support](https://spacecorps.github.io/contact.html)
- [Privacy Policy](https://spacecorps.github.io/privacy.html)
- [Authentication Standard](https://spacecorps.github.io/auth.md)
- [Agent LLM Manifest (llms.txt)](https://spacecorps.github.io/llms.txt)
- [Full LLM Manual (llms-full.txt)](https://spacecorps.github.io/llms-full.txt)
- [GitHub Organization](https://github.com/SpaceCorps)
