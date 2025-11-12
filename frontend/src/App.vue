<!-- src/App.vue -->
<template>
  <div id="app">
    <router-view />
    
    <!-- Componentes globales -->
    <AppNotification />
    <AppConfirmDialog />
  </div>
</template>

<script setup lang="ts">
import AppNotification from './components/AppNotification.vue'
import AppConfirmDialog from './components/AppConfirmDialog.vue'
</script>

<style>
/* Estilos globales */
</style>