<script lang="ts">
	import type { BankAccountView } from 'src/bindings/BankAccountView';
	let bankAccount: BankAccountView = {
		account_id: '',
		balance: 0,
		written_checks: [],
		account_transactions: []
	};
	let accountId = '';
	let jsonResponse = '';
	export async function getBankAccount() {
		const response = await fetch(`http://localhost:4005/bank-accounts/${accountId}`);
		const account = await response.json();
		console.log(account);
		jsonResponse = JSON.stringify(account, null, 2);
		bankAccount = account;
	}
</script>

<div class="form-control">
	<label class="input-group">
		<span>Your Account ID:</span>
		<input
			type="text"
			placeholder="Enter your account ID"
			class="input input-bordered"
			bind:value={accountId}
			on:keyup={async (e) => {
				await getBankAccount();
			}}
		/>
	</label>
</div>
<div class="card w-96 bg-base-100 shadow-xl">
	<div class="card-body">
		<p>Balance: {bankAccount.balance}</p>
		<br />
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
