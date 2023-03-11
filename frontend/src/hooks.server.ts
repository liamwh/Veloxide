/* eslint-disable */
// @ts-nocheck
import { SvelteKitAuth } from "@auth/sveltekit"
import GitHub from '@auth/core/providers/github';
import GoogleProvider from '@auth/core/providers/google';
import { sequence } from '@sveltejs/kit/hooks';
import { redirect, type Handle } from '@sveltejs/kit';
import { GITHUB_ID, GITHUB_SECRET, GOOGLE_CLIENT_ID, GOOGLE_CLIENT_SECRET } from "$env/static/private"

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
  providers: [
    GitHub({ clientId: GITHUB_ID, clientSecret: GITHUB_SECRET }),
    GoogleProvider({
      clientId: GOOGLE_CLIENT_ID,
      clientSecret: GOOGLE_CLIENT_SECRET
    })
  ]
}), authorization);