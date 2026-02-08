declare module '*.vue' {
  import { DefineComponent } from 'vue';
  const component: DefineComponent<{}, {}, any>;
  export default component;
}
declare module '*.svg' {
  const content: string;
  export default content;
}
declare module '*.yml' {
  const content: string;
  export default content;
}
declare module '*.yml?raw' {
  const content: string;
  export default content;
}
