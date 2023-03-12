<script lang="ts">
	export let accountId: string;
	import type { BankAccountCommand } from 'src/bindings/BankAccountCommand';
	import { PUBLIC_BANK_ACCOUNT_SERVICE_API_URL } from '$env/static/public';
	import { toast } from '@zerodevx/svelte-toast';

	let amount: number;
	let atmId: string;
	export async function withdrawMoney() {
		let command: BankAccountCommand = {
			WithdrawMoney: {
				atm_id: atmId,
				amount: amount
			}
		};
		let body = JSON.stringify(command, null, 2);
		const response = await fetch(
			`${PUBLIC_BANK_ACCOUNT_SERVICE_API_URL}/bank-accounts/${accountId}`,
			{
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(command, null, 2)
			}
		);
		switch (response.status) {
			case 200:
				return;
			case 404:
				toast.push('Account not found');
				return;
			default:
				toast.push('Error fetching account balance');
				return;
		}
	}
</script>

<div class="card w-96 bg-base-100 shadow-xl">
	<div class="card-body">
		<div class="form-control">
			<h2 class="card-title">Withdraw Money</h2>
			<div class="grid grid-flow-row auto-rows-max space-y-1">
				<!-- svelte-ignore a11y-label-has-associated-control -->
				<label class="label">
					<span class="label-text">Enter amount to withdraw</span>
				</label>
				<label class="input-group">
					<span>ATM ID</span>
					<input type="text" placeholder="atm-1" class="input input-bordered" bind:value={atmId} />
				</label>
				<label class="input-group">
					<input
						type="number"
						placeholder="0.01"
						class="input input-bordered"
						bind:value={amount}
					/>
					<span>EUR</span>
				</label>
				<!-- svelte-ignore a11y-click-events-have-key-events -->
				<div class="btn btn-primary" on:click={withdrawMoney}>Withdraw Money</div>
			</div>
		</div>
	</div>
</div>
