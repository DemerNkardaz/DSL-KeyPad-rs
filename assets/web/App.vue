<template>
	<div style="font-family: sans-serif; text-align: center; margin-top: 50px">
		<h1>Привет, Vue + Vite + Rust!</h1>
		<p>Это тестовый компонент App.vue без внешних ссылок.</p>

		<button @click="count++">Кликнули {{ count }} раз</button>
	</div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';

export default defineComponent({
	name: 'App',
	setup() {
		const count = ref(0);
		return { count };
	},
});
</script>

<style scoped>
button {
	padding: 10px 20px;
	font-size: 16px;
	cursor: pointer;
	border: none;
	border-radius: 5px;
	background-color: #42b983;
	color: white;
	transition: background-color 0.2s;
}
button:hover {
	background-color: #369770;
}
</style>
