---
title: Authenticate with Zed
description: "Authentication availability and API-key providers."
---

# Authenticate with Zed

Interactive sign-in is unavailable in this build, including sign-in to external services. You can use the editor without an account and [configure your own API keys](./ai/use-api-access.md) for AI features. Existing authenticated connections remain supported.

## What Features Require Signing In?

1. All real-time [collaboration features](./collaboration/overview.md).
2. [LLM-powered features](./ai/overview.md), if you are using Zed as the provider of your LLM models. To use AI without signing in, you can [bring and configure your own API keys](./ai/use-api-access.md).

## Signing Out

To sign out of Zed, you can use either of these methods:

- Click on the profile icon in the upper right corner and select `Sign Out` from the dropdown menu.
- Open the command palette and run the {#action client::SignOut} command.

## Email Addresses {#email}

Your Zed account's email address is the address provided by GitHub OAuth. If you have a public email address then it will be used, otherwise your primary GitHub email address will be used. Changes to your email address on GitHub can be synced to your Zed account by [signing in to zed.dev](https://zed.dev/sign_in).

Stripe is used for billing, and will use your Zed account's email address when starting a subscription. Changes to your Zed account email address do not currently update the email address used in Stripe. See [Updating Billing Information](./account/billing.md#updating-billing-info) for how to change this email address.
