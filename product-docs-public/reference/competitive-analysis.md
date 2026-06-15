# Competitive Analysis: Interactive Forms for CLI Command Building

**Date**: 2026-06-15
**Author**: AI Agent (via competitive research)
**Product**: terminal-expander — terminal-native text expander with Espanso-compatible forms

---

## Executive Summary

**No competitor exists** that combines multi-field forms + CLI command building + commercial packs. This is a genuine market gap.

---

## Competitor Landscape

### 1. navi (denisidoro/navi) — 17,200+ ⭐
**Type**: Interactive cheatsheet tool
**Language**: Rust
**Repo**: https://github.com/denisidoro/navi

| Feature | Support |
|---------|---------|
| Multi-field forms | ❌ — single-variable prompts only (sequential fzf) |
| CLI-focused | ✅ |
| Commercial packs | ❌ — open source, free community cheatsheets |
| Form UI style | One variable at a time via fzf picker, not a multi-field form |

**Verdict**: Variable substitution yes. Multi-field form — **No**. Commercial packs — **No**.

---

### 2. pet (knqyf263/pet) — 5,300+ ⭐
**Type**: CLI snippet manager
**Language**: Go
**Repo**: https://github.com/knqyf263/pet

| Feature | Support |
|---------|---------|
| Multi-field forms | ❌ — basic `<param>` substitution only |
| CLI-focused | ✅ |
| Commercial packs | ❌ — open source, sync via Gist/GitLab |
| Form UI style | Single param at a time via fzf/peco |

**Verdict**: Basic variables yes. Multi-field form — **No**. Commercial packs — **No**.

---

### 3. Fig / Amazon Q Developer CLI — 25,200+ ⭐
**Type**: CLI autocomplete
**Language**: TypeScript
**Repo**: https://github.com/withfig/autocomplete (archived, acquired by Amazon)

| Feature | Support |
|---------|---------|
| Multi-field forms | ❌ — autocomplete inline suggestions only |
| CLI-focused | ✅ |
| Commercial packs | ❌ — open source specs, not sold |
| Form UI style | IDE-style autocomplete dropdown, not forms |

**Verdict**: Autocomplete yes. Multi-field form — **No**. Commercial packs — **No**.

---

### 4. Warp (warpdotdev/warp) — 61,700+ ⭐
**Type**: Modern terminal / AI-powered dev environment
**Language**: Rust
**Repo**: https://github.com/warpdotdev/warp

| Feature | Support |
|---------|---------|
| Multi-field forms | ❌ — AI agent + smart suggestions, not forms |
| CLI-focused | ✅ (terminal IDE) |
| Commercial packs | ❌ — SaaS platform (free + enterprise), no command pack product |
| Form UI style | AI command generation, no structured forms |

**Verdict**: AI suggestions yes. Multi-field form — **No**. Commercial packs — **No**.

---

### 5. Espanso (espanso/espanso) — 13,900+ ⭐
**Type**: Cross-platform text expander
**Language**: Rust
**Repo**: https://github.com/espanso/espanso
**Forms docs**: https://espanso.org/docs/matches/forms/

| Feature | Support |
|---------|---------|
| Multi-field forms | ✅ **Full support** — text, choice, list, multiline |
| CLI-focused | ❌ — generic text expander, not CLI-specific |
| Commercial packs | ❌ — open source, community hub (hub.espanso.org), donation-supported |
| Form UI style | **GUI popup** (wxWidgets), NOT terminal-native |

**Verdict**: Full multi-field forms — **YES**. CLI-focused — **No** (general purpose).
This is the closest parallel but fundamentally different: GUI forms vs terminal-native.
Also no commercial model around CLI command packs.

---

### 6. terminal-expander (donnyaw/terminal-expander) — your product
**Type**: Terminal-native text expander with forms
**Language**: Rust
**Repo**: https://github.com/donnyaw/terminal-expander

| Feature | Support |
|---------|---------|
| Multi-field forms | ✅ — Cursive TUI, text/choice/list/multiline |
| CLI-focused | ✅ — specifically designed for terminal CLI use |
| Commercial packs | **Planned** — one-time purchase form pack model |
| Form UI style | **In-terminal TUI** (Cursive), no GUI popup |

**Verdict**: Only product that combines multi-field forms + CLI focus + potential commercial model.

---

### 7. Reference Tools (no interactivity)
- **tldr** (https://github.com/tldr-pages/tldr) — Community-maintained simplified man pages. Read-only. No forms. No commercial.
- **cheat** (https://github.com/cheat/cheat) — CLI cheatsheets. Read-only. No forms. No commercial.
- **cheat.sh** (https://github.com/chubin/cheat.sh) — Online cheat server. Read-only. No forms. No commercial.

---

## Market Gap Analysis

| Requirement | navi | pet | Fig/Amazon Q | Warp | Espanso | **terminal-expander** |
|------------|------|-----|-------------|------|---------|----------------------|
| Multi-field forms | ❌ | ❌ | ❌ | ❌ | ✅ GUI | ✅ **TUI** |
| CLI-command focused | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ |
| Terminal-native form UI | ✅ (fzf) | ✅ (fzf) | ❌ | ❌ | ❌ | ✅ **Cursive** |
| Commercial form packs | ❌ | ❌ | ❌ | ❌ | ❌ | **Planned** |
| Trigger-based expansion | ❌ | ❌ | ❌ | ❌ | ✅ | ✅ |
| Variable system | ✅ single | ✅ single | ❌ | AI | ✅ | ✅ |

**Gap**: No existing tool combines ALL of:
1. Multi-field forms (text/choice/list displayed simultaneously)
2. CLI-command focused (not generic text expansion)
3. Terminal-native (no GUI popup)
4. Commercial pack model (one-time purchase per category)

---

## Competitive Advantage Summary

| Advantage | Detail |
|-----------|--------|
| **Multi-field forms in terminal** | Only tool that renders Espanso-style forms via Cursive TUI (not GUI popup) |
| **CLI-command focus** | Designed for building CLI commands, not generic text snippets |
| **Espanso-compatible YAML** | Users can reuse existing Espanso match files |
| **Shell plugin system** | Bash auto-expand, zle widget, fish hooks |
| **Clean market** | No competitor selling CLI form packs — you're first to market |
| **Low barrier to entry** | Packs are just `.yml` files — low production cost, high perceived value |

---

## Recommended Business Model

| Component | Detail |
|-----------|--------|
| **Free tier** | Open-source tool + basic form packs (find, fd, locate) |
| **Paid packs** | One-time payment per category: Docker ($5), Git ($5), Kubernetes ($10), AWS ($10), System Admin ($5) |
| **Bundle** | All packs for $25-30 |
| **Distribution** | `texpand install <pack>` fetches `.yml` from registry |
| **Payment** | Gumroad / Lemon Squeezy for one-time license keys |
| **Enterprise** | Custom form packs for internal CLIs — private registry |

---

## Sources

- navi: https://github.com/denisidoro/navi
- pet: https://github.com/knqyf263/pet
- Fig/Amazon Q: https://github.com/withfig/autocomplete
- Warp: https://github.com/warpdotdev/warp
- Espanso: https://github.com/espanso/espanso | https://espanso.org/docs/matches/forms/
- tldr: https://github.com/tldr-pages/tldr
- cheat: https://github.com/cheat/cheat
- terminal-expander: https://github.com/donnyaw/terminal-expander
