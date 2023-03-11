<script lang="ts">
	import { page } from '$app/stores';
	import Avatar from './Avatar.svelte';
</script>

<div class="navbar bg-base-100 w-screen">
	<div class="navbar-start">
		<div class="dropdown">
			<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
			<!-- svelte-ignore a11y-label-has-associated-control -->
			<label tabindex="0" class="btn btn-ghost btn-circle">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="h-5 w-5"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
					><path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M4 6h16M4 12h16M4 18h7"
					/></svg
				>
			</label>
			<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
			<ul
				tabindex="0"
				class="menu menu-compact dropdown-content mt-3 p-2 shadow bg-base-100 rounded-box w-52"
			>
				<li><a href="/protected">Protected</a></li>
			</ul>
		</div>
	</div>
	<div class="navbar-center">
		<a href="/" class="btn btn-ghost normal-case text-xl">ExampleBanking: An example Veloxide App</a
		>
	</div>
	<div class="navbar-end">
		<div class="dropdown dropdown-end">
			<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
			<!-- svelte-ignore a11y-label-has-associated-control -->
			<label tabindex="0" class="btn btn-ghost btn-circle avatar">
				<div class="w-10 rounded-full">
					<Avatar />
				</div>
			</label>
			<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
			<ul
				tabindex="0"
				class="mt-3 p-2 shadow menu menu-compact dropdown-content bg-base-100 rounded-box w-52"
			>
				<li>
					<!-- svelte-ignore a11y-missing-attribute -->
					<a class="justify-between" href="/profile"> Profile </a>
				</li>
				<!-- svelte-ignore a11y-missing-attribute -->
				{#if $page.data.session}
					<li><a href="/auth/signout" data-sveltekit-preload-data="off">Logout</a></li>
				{:else}
					<li><a href="/auth/signin" data-sveltekit-preload-data="off">Login</a></li>
				{/if}
			</ul>
		</div>
	</div>
</div>
