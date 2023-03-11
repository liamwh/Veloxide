<script lang="ts">
	import MyAccount from '$lib/MyAccount.svelte';
	let accountId: string;
</script>

<div class="form-control">
	<label class="input-group">
		<span>Enter an Account ID:</span>
		<input
			type="text"
			placeholder="Enter an account ID"
			class="input input-bordered"
			bind:value={accountId}
		/>
	</label>
</div>
<br />
<MyAccount {accountId} />
