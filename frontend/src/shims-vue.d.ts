// Shims mínimos para SFCs de Vue para que TypeScript pueda importar archivos .vue
declare module '*.vue' {
  import { DefineComponent } from 'vue'
  const component: DefineComponent<{}, {}, {}, any>
  export default component
}
