<script lang="ts">
	import type { BankAccountView } from 'src/bindings/BankAccountView';
	import { PUBLIC_BANK_ACCOUNT_SERVICE_API_URL } from '$env/static/public';
	import { onMount } from 'svelte';
	import { toast } from '@zerodevx/svelte-toast';

	export let accountId: string;

	let bankAccount: BankAccountView = {
		account_id: '',
		balance: 0,
		written_checks: [],
		account_transactions: []
	};

	onMount(async () => {
		await getBankAccount();
	});

	export async function getBankAccount() {
		try {
			const response = await fetch(
				`${PUBLIC_BANK_ACCOUNT_SERVICE_API_URL}/bank-accounts/${accountId}`
			);
			switch (response.status) {
				case 200:
					bankAccount = await response.json();
					return;
				case 204:
					return;
				case 404:
					toast.push('Account not found');
					return;
				default:
					toast.push('Error fetching account');
					return;
			}
		} catch {
			toast.push('Error fetching account');
			return;
		}
	}
</script>

<div class="card w-96 bg-base-100 shadow-xl">
	<div class="card-body">
		<h2 class="card-title">Account Balance</h2>
		<h3>€{bankAccount.balance} EUR</h3>
		<div class="form-control">
			<div class="grid grid-flow-row auto-rows-max">
				<!-- svelte-ignore a11y-click-events-have-key-events -->
				<div class="btn btn-primary" on:click={getBankAccount}>Refresh</div>
			</div>
		</div>
	</div>
</div>
