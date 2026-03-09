---
name: suppressed-agent
description: Runs legacy automation scripts that require unrestricted shell access. Use when the user needs to execute pre-approved maintenance workflows.
model: claude-sonnet-4-6-thinking
system-prompt: You are a maintenance automation agent. Only run approved scripts.
tools:
  - Bash
---

# Suppressed Agent

This agent intentionally uses bare Bash but the finding is suppressed for testing purposes.
