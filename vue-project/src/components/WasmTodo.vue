<script setup lang="ts">

import init, { TodoList } from '../../../pkg/wasm_todo'
import { ref } from 'vue'

await init();
console.log(TodoList)
const todoList = TodoList.new("First Todo List")
todoList.add('Hello from Vue!')
todoList.add('Hello from Rust!')
todoList.add('Hello from WebAssembly!')
let len = ref(todoList.len())
console.log(len.value);
console.log(todoList.get_by_index(0)?.title())

let is_checked_list = ref([false, false, false])

const toggle_todo = (index: number, is_checked: boolean) => {
  console.log(index, is_checked);
  todoList.set_completed(index, is_checked)
  console.log(todoList.get_by_index(index)?.completed())
}
let new_todo = ref("");

const add_todo = () => {
  todoList.add(new_todo.value)
  len.value = todoList.len()
  is_checked_list.value.push(false)
  new_todo.value = ""
}
</script>

<template>
  <h2>{{ todoList.list_name() }}</h2>
  <input type="text" v-model="new_todo" />
  <button @click="add_todo()">Add Todo</button>
  <div v-for="idx in len" :key="idx">
    <input type="checkbox"  v-model="is_checked_list[idx-1]" @change="toggle_todo(idx-1, is_checked_list[idx-1])" />
    {{ todoList.get_by_index(idx - 1)?.title() }}
    <button @click="todoList.remove(idx - 1)">Remove</button>
  </div>
</template>