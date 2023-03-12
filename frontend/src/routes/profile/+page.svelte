<script lang="ts">
	import { page } from '$app/stores';
	import Avatar from '$lib/Avatar.svelte';
</script>

{#if $page.data.session}
	<div class="flex justify-center w-screen">
		<div class="card w-96 bg-base-100 shadow-xl">
			<figure><Avatar /></figure>
			<div class="card-body">
				<h2 class="card-title">
					{$page.data.session.user?.name}
				</h2>
				<p>{$page.data.session.user?.email}</p>
				<div class="card-actions justify-end">
					<a href="/auth/signout" class="btn btn-primary" data-sveltekit-preload-data="off"
						>Sign out</a
					>
				</div>
			</div>
		</div>
	</div>
{:else}
	<div class="flex justify-center w-screen">
		<div class="card w-96 bg-base-100 shadow-xl">
			<div class="card-body">
				<div class="flex-auto justify-center">
					<div><h2 class="card-title">You are not logged in</h2></div>
					<div>
						<p>
							<a href="/auth/signin" class="btn btn-primary" data-sveltekit-preload-data="off"
								>Sign in</a
							>
						</p>
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}
