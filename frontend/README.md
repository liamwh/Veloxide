<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD041 -->

<p align="center">
    <a href="https://github.com/liamwh/Veloxide"><img src="../docs/.assets/veloxide-banner.png" alt="Veloxide"
            width=100%></a>
    <p align="center">
</p>

---

## Frontend Development

## Requirements

- [Node.js](https://nodejs.org/en/)
- A package manager, [pnpm](https://pnpm.io/) is recommended

## Getting started

Install dependencies with `pnpm install`, then start a development server:

```bash
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open
```

## Authentication

The frontend uses OAuth to authenticate. This is done using Auth.js. See more [here](https://authjs.dev/getting-started/oauth-tutorial).

## Building

To create a production version of your app:

```bash
npm run build
```

You can preview the production build with `npm run preview`.

> To deploy your app, you may need to install an [adapter](https://kit.svelte.dev/docs/adapters) for your target environment.
