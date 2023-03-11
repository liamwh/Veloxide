<script lang="ts">
	import DepositMoney from './DepositMoney.svelte';
	import AccountTransactionsCard from './AccountTransactionsCard.svelte';
	import WithdrawMoney from './WithdrawMoney.svelte';
	import AccountBalanceCard from './AccountBalanceCard.svelte';

	export let accountId: string;
</script>

<div class="flex justify-center items-center w-screen">
	<article class="prose lg:prose">
		<h2>Your Account ID: {accountId}</h2>
	</article>
</div>
<div class="flex flex-wrap justify-center w-screen">
	<div class="flex flex-wrap space-x-2 space-y-2">
		<AccountBalanceCard {accountId} />
		<DepositMoney {accountId} />
		<WithdrawMoney {accountId} />
		<AccountTransactionsCard {accountId} />
	</div>
</div>
