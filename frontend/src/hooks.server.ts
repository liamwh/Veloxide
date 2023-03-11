/* eslint-disable */
// @ts-nocheck
import { SvelteKitAuth } from "@auth/sveltekit"
import GitHub from '@auth/core/providers/github';
import GoogleProvider from '@auth/core/providers/google';
import { GITHUB_ID, GITHUB_SECRET, GOOGLE_CLIENT_ID, GOOGLE_CLIENT_SECRET } from "$env/static/private"

export const handle = SvelteKitAuth({
  providers: [
    GitHub({ clientId: GITHUB_ID, clientSecret: GITHUB_SECRET }),
    GoogleProvider({
      clientId: GOOGLE_CLIENT_ID,
      clientSecret: GOOGLE_CLIENT_SECRET
    })
  ]
});