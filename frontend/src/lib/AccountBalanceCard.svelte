<script lang="ts">
	import type { BankAccountView } from 'src/bindings/BankAccountView';
	import { onMount } from 'svelte';

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
		const response = await fetch(`http://localhost:4005/bank-accounts/${accountId}`);
		const account = await response.json();
		console.log(account);
		if (response.status === 200) {
			bankAccount = account;
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
