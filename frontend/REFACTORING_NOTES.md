# Refactorización de BalanceForm

## 📋 Resumen

Se refactorizó el componente `BalanceForm.vue` (anteriormente ~650 líneas) dividiéndolo en múltiples componentes y composables reutilizables para mejorar la mantenibilidad, legibilidad y escalabilidad del código.

## 🎯 Objetivos Logrados

- ✅ **Separación de responsabilidades**: Cada componente tiene una única responsabilidad
- ✅ **Reutilización de código**: Componentes genéricos que se pueden usar en otros contextos
- ✅ **Mejora de mantenibilidad**: Código más fácil de entender y modificar
- ✅ **TypeScript mejorado**: Tipado más específico y seguro
- ✅ **Reducción de complejidad**: El componente principal ahora tiene ~150 líneas vs 650

## 📁 Estructura Nueva

```
frontend/src/
├── composables/
│   └── useBalanceForm.ts          # Lógica de negocio del formulario
├── components/
│   ├── BalanceConfigCard.vue      # Card de configuración del balance
│   ├── BalanceWeekTable.vue       # Tabla reutilizable para semanas
│   ├── BalanceFinalTable.vue      # Tabla de consultas y exámenes
│   └── CalculationsTable.vue      # Tabla de cálculos y coeficientes
└── views/
    ├── BalanceForm.vue            # Componente principal refactorizado
    └── BalanceForm.vue.backup     # Versión original (respaldo)
```

## 🔧 Componentes Creados

### 1. `useBalanceForm.ts` (Composable)

**Responsabilidad**: Encapsular toda la lógica de estado y métodos del formulario de balance.

**Características**:
- Manejo de estado (loading, modales, tabs)
- Inicialización del balance
- Operaciones CRUD sobre asignaturas
- Cálculos y guardado
- Gestión de advertencias de cambios sin guardar

**Exports**:
```typescript
{
  // State
  activeTab, isSaving, showAddSubjectModal, customSubjectName, tabs,
  
  // Stores
  balanceStore, asignaturasStore, uiStore,
  
  // Methods
  initializeBalance, markDirty, updateCellValue, calculateBalance,
  saveBalance, openAddSubjectModal, closeAddSubjectModal,
  addExistingSubject, addCustomSubject, confirmDeleteSubject,
  setupUnsavedWarning
}
```

### 2. `BalanceConfigCard.vue`

**Responsabilidad**: Card con los controles de configuración del balance (año académico, período, etc.) y botones de acción.

**Props**:
```typescript
{
  config: BalanceConfig,
  isSaving: boolean,
  hasUnsavedChanges: boolean
}
```

**Eventos**:
- `update:config`: Actualización de un campo de configuración
- `calculate`: Ejecutar cálculos
- `save`: Guardar balance

### 3. `BalanceWeekTable.vue`

**Responsabilidad**: Tabla reutilizable para mostrar semanas del balance con inputs editables.

**Props**:
```typescript
{
  subjects: Subject[],
  title: string,
  weeks: number[],
  startCellIndex: number,
  columnsPerWeek?: number,      // default: 4
  headerIcon?: string,           // default: '📅'
  colorScheme?: 'blue' | 'purple' | 'green',  // default: 'blue'
  showActions?: boolean          // default: false
}
```

**Características**:
- Esquemas de color configurables
- Cálculo automático del número de celdas
- Resaltado de celdas con valores
- Estilos responsive

**Eventos**:
- `update-value`: Cambio en una celda
- `delete-subject`: Eliminar asignatura (si showActions=true)

### 4. `BalanceFinalTable.vue`

**Responsabilidad**: Tabla específica para consultas y exámenes finales con botón de eliminar.

**Props**:
```typescript
{
  subjects: Subject[]
}
```

**Eventos**:
- `update-value`: Cambio en una celda
- `delete-subject`: Eliminar asignatura

### 5. `CalculationsTable.vue`

**Responsabilidad**: Mostrar la tabla de cálculos y coeficientes.

**Props**:
```typescript
{
  calculations: Calculation[]
}
```

**Características**:
- Estado vacío con mensaje descriptivo
- Tabla responsive con scroll horizontal
- Columnas: Total, C, CP, S, PL, TE, T, PP, Coef.

## 🎨 Mejoras de Diseño

### Esquema de Colores Configurable

Las tablas ahora soportan múltiples esquemas de color:

- **Blue**: Semanas 1-12 (estándar)
- **Purple**: Semanas 13-15
- **Green**: Consultas y exámenes finales

Cada esquema incluye:
- Border color
- Header gradient
- Focus ring
- Hover effects
- Cell highlight cuando tiene valor

### Componentes más Pequeños y Manejables

**Antes**: 1 archivo monolítico de 650 líneas
**Ahora**: 6 archivos con responsabilidades claras

| Archivo | Líneas | Responsabilidad |
|---------|--------|----------------|
| BalanceForm.vue | ~150 | Orquestación y layout |
| useBalanceForm.ts | ~170 | Lógica de negocio |
| BalanceConfigCard.vue | ~100 | Configuración |
| BalanceWeekTable.vue | ~150 | Tabla de semanas |
| BalanceFinalTable.vue | ~120 | Tabla final |
| CalculationsTable.vue | ~60 | Cálculos |

## 🔄 Uso en el Componente Principal

```vue
<template>
  <!-- Antes: Todo inline con lógica mezclada -->
  
  <!-- Ahora: Componentes semánticos -->
  <BalanceConfigCard
    :config="balanceConfig"
    :is-saving="isSaving"
    :has-unsaved-changes="balanceStore.hasUnsavedChanges"
    @update:config="handleConfigUpdate"
    @calculate="calculateBalance"
    @save="saveBalance"
  />

  <BalanceWeekTable
    :subjects="balanceStore.currentBalance.subjects"
    title="Semanas 1 - 4"
    :weeks="[1, 2, 3, 4]"
    :start-cell-index="0"
    color-scheme="blue"
    @update-value="updateCellValue"
  />
</template>

<script setup lang="ts">
// Antes: ~50 líneas de lógica mezclada con UI

// Ahora: Composable limpio
const {
  activeTab,
  balanceStore,
  calculateBalance,
  saveBalance,
  // ... resto de funcionalidad
} = useBalanceForm()
</script>
```

## 🧪 Testing

Los componentes ahora son mucho más fáciles de testear:

```typescript
// Testear BalanceWeekTable de forma aislada
test('should emit update-value when input changes', () => {
  const wrapper = mount(BalanceWeekTable, {
    props: {
      subjects: mockSubjects,
      title: 'Test',
      weeks: [1, 2],
      startCellIndex: 0
    }
  })
  // Test específico sin dependencias del componente padre
})

// Testear lógica del composable sin UI
test('useBalanceForm should initialize balance', () => {
  const { initializeBalance, balanceStore } = useBalanceForm()
  initializeBalance()
  expect(balanceStore.currentBalance).toBeDefined()
})
```

## 📊 Beneficios

### Mantenibilidad
- Cada archivo tiene una responsabilidad clara
- Cambios localizados (modificar tabla no afecta configuración)
- Más fácil encontrar y corregir bugs

### Reutilización
- `BalanceWeekTable` puede usarse en otros contextos
- `useBalanceForm` puede adaptarse para otros formularios similares
- Componentes más pequeños = más oportunidades de reutilización

### Escalabilidad
- Agregar nuevas features es más simple
- Fácil agregar nuevos tipos de tablas o esquemas de color
- Composables pueden compartirse entre diferentes vistas

### Developer Experience
- Autocompletado TypeScript mejorado
- Props y eventos claramente definidos
- Navegación más fácil entre archivos relacionados
- Menos scroll, más contexto visible

## 🚀 Próximos Pasos Sugeridos

1. **Tests unitarios**: Agregar tests para cada componente y el composable
2. **Storybook**: Documentar componentes visuales en Storybook
3. **Optimización**: Implementar virtualización para tablas grandes
4. **Exportación**: Implementar funcionalidad de exportar a Excel
5. **Validación**: Agregar validaciones de datos en los inputs
6. **Accesibilidad**: Mejorar labels y ARIA attributes

## 📝 Notas de Migración

- El archivo original se guardó como `BalanceForm.vue.backup`
- No hay cambios en la API pública del componente
- No requiere cambios en rutas o stores
- Compatible con el resto del sistema

## 🐛 Debugging

Si encuentras problemas:

1. Revisa la consola del navegador para errores TypeScript
2. Verifica que todos los imports estén correctos
3. Comprueba que los stores tengan los datos esperados
4. Usa Vue DevTools para inspeccionar props y eventos
5. Consulta el archivo `.backup` para comparar comportamiento

## 👥 Contribuir

Al agregar nuevas features:
- Mantén componentes pequeños (< 200 líneas)
- Usa composables para lógica compleja
- Define tipos TypeScript claros
- Documenta props y eventos
- Sigue el patrón de nombres consistente
