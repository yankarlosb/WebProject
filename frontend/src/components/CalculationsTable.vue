<template>
  <div class="p-6">
    <h3 class="text-lg font-bold text-blue-700 mb-4">Cálculos y Coeficientes</h3>

    <!-- Estado vacío -->
    <div v-if="calculations.length === 0" class="text-center py-12 bg-gray-50 rounded-lg">
      <p class="text-gray-600">No hay cálculos disponibles. Presiona "Calcular" primero.</p>
    </div>

    <!-- Tabla de cálculos -->
    <div v-else class="overflow-x-auto">
      <table class="min-w-full divide-y divide-blue-100 border border-blue-100 rounded-lg">
        <thead class="bg-blue-50">
          <tr>
            <th class="px-6 py-4 text-left text-sm font-bold text-blue-700">Asignatura</th>
            <th class="px-4 py-4 text-center text-sm font-bold text-blue-700">Total</th>
            <th class="px-4 py-4 text-center text-sm font-bold text-blue-700">C</th>
            <th class="px-4 py-4 text-center text-sm font-bold text-blue-700">CP</th>
            <th class="px-4 py-4 text-center text-sm font-bold text-blue-700">S</th>
            <th class="px-4 py-4 text-center text-sm font-bold text-blue-700">PL</th>
            <th class="px-4 py-4 text-center text-sm font-bold text-blue-700">TE</th>
            <th class="px-4 py-4 text-center text-sm font-bold text-blue-700">T</th>
            <th class="px-4 py-4 text-center text-sm font-bold text-blue-700">PP</th>
            <th class="px-4 py-4 text-center text-sm font-bold text-blue-700">Coef.</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-blue-50">
          <tr v-for="calc in calculations" :key="calc.subjectId" class="hover:bg-blue-50 transition">
            <td class="px-6 py-4 font-semibold text-blue-900">{{ calc.subjectName }}</td>
            <td class="px-4 py-4 text-center text-blue-800">{{ calc.total }}</td>
            <td class="px-4 py-4 text-center text-blue-800">{{ calc.C }}</td>
            <td class="px-4 py-4 text-center text-blue-800">{{ calc.CP }}</td>
            <td class="px-4 py-4 text-center text-blue-800">{{ calc.S }}</td>
            <td class="px-4 py-4 text-center text-blue-800">{{ calc.PL }}</td>
            <td class="px-4 py-4 text-center text-blue-800">{{ calc.TE }}</td>
            <td class="px-4 py-4 text-center text-blue-800">{{ calc.T }}</td>
            <td class="px-4 py-4 text-center text-blue-800">{{ calc.PP }}</td>
            <td class="px-4 py-4 text-center font-bold text-blue-900">{{ calc.coef }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Calculation {
  subjectId: string
  subjectName: string
  total: number
  C: number
  CP: number
  S: number
  PL: number
  TE: number
  T: number
  PP: number
  coef: number
}

interface Props {
  calculations: Calculation[]
}

defineProps<Props>()
</script>
