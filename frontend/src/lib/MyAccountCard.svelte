<script lang="ts">
	import DepositMoneyCard from './DepositMoneyCard.svelte';
	import AccountTransactionsCard from './AccountTransactionsCard.svelte';
	import WithdrawMoneyCard from './WithdrawMoneyCard.svelte';
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
		<DepositMoneyCard {accountId} />
		<WithdrawMoneyCard {accountId} />
		<AccountTransactionsCard {accountId} />
	</div>
</div>
