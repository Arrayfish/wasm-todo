<script setup lang="ts">

import init, { TodoList } from '../../../pkg/wasm_todo'
import { ref } from 'vue'

await init();
console.log(TodoList)
const todoList = TodoList.new()
todoList.add('Hello from Vue!')
todoList.add('Hello from Rust!')
todoList.add('Hello from WebAssembly!')
let len = ref(todoList.len())
console.log(len.value);
console.log(todoList.get(0)?.title())
</script>

<template>
    <ul>
      <li v-for="i in 3" :key="i">
        {{ todoList.get(i - 1)?.title() }}
      </li>
    </ul>  
</template>