<script setup lang="ts">
defineProps<{
  msg: string
}>()

import {TodoList} from '../../../pkg/wasm_todo'

const todoList = TodoList.new()
todoList.add('Hello from Vue!')
todoList.add('Hello from Rust!')
todoList.add('Hello from WebAssembly!')

console.log(todoList.todos())
</script>

<template>
  <ul>
    <li v-for="todo in todoList.todos" :key="todo.id">
      {{ todo.title }}
    </li>
  </ul>  
</template>