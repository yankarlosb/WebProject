<template>
  <div class="border-2 border-green-200 rounded-lg bg-white overflow-hidden shadow-sm hover:shadow-md transition-shadow">
    <div class="bg-gradient-to-r from-green-600 to-green-500 px-4 py-2.5 flex items-center justify-between">
      <span class="text-sm font-semibold text-white">✅ Consultas y Exámenes Finales</span>
      <span class="text-xs text-green-100">9 celdas por asignatura</span>
    </div>
    
    <div class="overflow-x-auto">
      <table class="w-full divide-y divide-gray-200">
        <thead class="bg-green-50">
          <tr>
            <th class="px-4 py-3 text-left text-xs font-bold text-green-700 uppercase min-w-[160px]">
              Asignatura
            </th>
            <th colspan="4" class="px-2 py-2 text-center text-xs font-semibold text-green-700 border-l border-gray-300">
              Consultas
            </th>
            <th colspan="5" class="px-2 py-2 text-center text-xs font-semibold text-green-700 border-l border-gray-300">
              Exámenes Finales
            </th>
            <th class="px-4 py-3 text-center text-xs font-bold text-red-700 uppercase border-l border-gray-300">
              Acciones
            </th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-100">
          <tr
            v-for="subject in subjects"
            :key="`final-${subject.id}`"
            class="hover:bg-green-50/50 transition-colors"
          >
            <td class="px-4 py-3 font-semibold text-sm text-gray-900 bg-gray-50">
              {{ subject.name }}
            </td>
            
            <!-- Consultas: 4 celdas (índices 60-63) -->
            <td
              v-for="cellIndex in 4"
              :key="`cons-${cellIndex}`"
              class="px-1 py-2 text-center"
              :class="cellIndex === 1 ? 'border-l border-gray-300' : ''"
            >
              <input
                type="number"
                :value="subject.values[59 + cellIndex]"
                @input="(e) => $emit('update-value', subject.id, 59 + cellIndex, e)"
                min="0"
                class="w-12 h-8 text-center border border-gray-200 rounded text-sm focus:ring-2 focus:ring-green-500 focus:border-green-500 outline-none"
                :class="(subject.values[59 + cellIndex] || 0) > 0 ? 'bg-green-50 font-semibold' : ''"
              />
            </td>
            
            <!-- Exámenes Finales: 5 celdas (índices 64-68) -->
            <td
              v-for="cellIndex in 5"
              :key="`exam-${cellIndex}`"
              class="px-1 py-2 text-center"
              :class="cellIndex === 1 ? 'border-l border-gray-300' : ''"
            >
              <input
                type="number"
                :value="subject.values[63 + cellIndex]"
                @input="(e) => $emit('update-value', subject.id, 63 + cellIndex, e)"
                min="0"
                class="w-12 h-8 text-center border border-gray-200 rounded text-sm focus:ring-2 focus:ring-green-500 focus:border-green-500 outline-none"
                :class="(subject.values[63 + cellIndex] || 0) > 0 ? 'bg-green-50 font-semibold' : ''"
              />
            </td>
            
            <!-- Botón eliminar -->
            <td class="px-4 py-3 text-center border-l border-gray-300">
              <button
                @click="$emit('delete-subject', subject.id)"
                class="inline-flex items-center gap-1 px-3 py-1.5 text-xs font-medium text-red-700 bg-red-50 hover:bg-red-100 rounded-lg transition-colors"
              >
                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                </svg>
                Eliminar
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Subject {
  id: string
  name: string
  values: number[]
}

interface Props {
  subjects: Subject[]
}

defineProps<Props>()

defineEmits<{
  'update-value': [subjectId: string, cellIndex: number, event: Event]
  'delete-subject': [subjectId: string]
}>()
</script>

<style scoped>
input[type="number"]::-webkit-inner-spin-button,
input[type="number"]::-webkit-outer-spin-button {
  opacity: 0;
}

input[type="number"]:hover::-webkit-inner-spin-button,
input[type="number"]:hover::-webkit-outer-spin-button {
  opacity: 1;
}

input[type="number"]:focus {
  transform: scale(1.05);
  transition: transform 0.15s ease;
}
</style>
