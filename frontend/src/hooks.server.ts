/* eslint-disable */
// @ts-nocheck
import { SvelteKitAuth } from "@auth/sveltekit"
import { sequence } from '@sveltejs/kit/hooks';
import { redirect, type Handle } from '@sveltejs/kit';
import { GITHUB_ID, GITHUB_SECRET, GOOGLE_CLIENT_ID, GOOGLE_CLIENT_SECRET, EMAIL_SERVER, EMAIL_FROM } from "$env/static/private"
import { PrismaAdapter } from "@next-auth/prisma-adapter"
import { PrismaClient } from "@prisma/client"

import GitHub from '@auth/core/providers/github';
import GoogleProvider from '@auth/core/providers/google';
import EmailProvider from "@auth/core/providers/email";


const prisma = new PrismaClient()

async function authorization({ event, resolve }) {
  // Protect any routes under /authenticated
  if (event.url.pathname.startsWith('/authenticated')) {
    const session = await event.locals.getSession();
    if (!session) {
      throw redirect(303, '/auth/signin');
    }
  }

  // If the request is still here, just proceed as normally
  const result = await resolve(event, {
    transformPageChunk: ({ html }) => html
  });
  return result;
}

export const handle: Handle = sequence(SvelteKitAuth({
  adapter: PrismaAdapter(prisma),
  // the session override fixes a weird bug in the adapter
  // src: https://github.com/nextauthjs/next-auth/issues/6076#issuecomment-1354087465
  session: {
    strategy: "database",
    generateSessionToken: () => {
      return crypto.randomUUID();
    }
  },
  providers: [
    GitHub({ clientId: GITHUB_ID, clientSecret: GITHUB_SECRET }),
    GoogleProvider({
      clientId: GOOGLE_CLIENT_ID,
      clientSecret: GOOGLE_CLIENT_SECRET,
    }),
    EmailProvider({
      server: EMAIL_SERVER,
      from: EMAIL_FROM,
    }
    )
  ]
}), authorization);