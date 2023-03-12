<script lang="ts">
	import type { BankAccountView } from 'src/bindings/BankAccountView';
	import { PUBLIC_BANK_ACCOUNT_SERVICE_API_URL } from '$env/static/public';
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
		const response = await fetch(
			`${PUBLIC_BANK_ACCOUNT_SERVICE_API_URL}/bank-accounts/${accountId}`
		);
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
		<h2 class="card-title">Account Tansactions</h2>
		<table class="table table-compact w-full">
			<thead>
				<tr>
					<th>Description</th>
					<th>Amount</th>
				</tr>
			</thead>
			<tbody>
				{#each bankAccount.account_transactions as transaction}
					<tr>
						<td>{transaction.description}</td>
						<td>{transaction.amount}</td>
					</tr>
				{/each}
			</tbody>

			<div class="card-actions justify-end">
				<button class="btn btn-primary" on:click={getBankAccount}> Refresh </button>
			</div>
		</table>
	</div>
</div>
