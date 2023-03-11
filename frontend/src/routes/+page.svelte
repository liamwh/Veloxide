<script>
	import OpenAccount from '$lib/OpenAccount.svelte';
	import GetBankAccount from '$lib/GetBankAccount.svelte';
	import { signIn, signOut } from '@auth/sveltekit/client';
	import { page } from '$app/stores';
</script>

<div><GetBankAccount /></div>
