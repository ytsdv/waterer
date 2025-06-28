<script lang="ts">
  import { cva, type VariantProps } from 'class-variance-authority';

  const buttonVariants = cva(
    // Base styles
    "px-5 py-3 rounded-lg font-medium transition-all duration-200 shadow-sm cursor-pointer border border-transparent disabled:opacity-60 disabled:cursor-not-allowed",
    {
      variants: {
        variant: {
          primary: "bg-primary-600 text-white hover:bg-primary-700 hover:enabled:-translate-y-0.5 hover:enabled:shadow-md",
          secondary: "bg-gray-200 text-gray-800 border-gray-300 hover:bg-gray-300 hover:enabled:-translate-y-0.5 hover:enabled:shadow-md dark:bg-gray-700 dark:text-gray-200 dark:border-gray-600 dark:hover:bg-gray-600",
          destructive: "bg-destructive-600 text-white hover:bg-destructive-700 hover:enabled:-translate-y-0.5 hover:enabled:shadow-md",
          outline: "border-gray-300 bg-transparent text-gray-700 hover:bg-gray-50 hover:enabled:-translate-y-0.5 hover:enabled:shadow-md dark:border-gray-600 dark:text-gray-200 dark:hover:bg-gray-800",
          ghost: "border-transparent bg-transparent text-gray-700 hover:bg-gray-100 hover:enabled:-translate-y-0.5 dark:text-gray-200 dark:hover:bg-gray-800"
        },
        size: {
          sm: "px-3 py-2 text-sm",
          md: "px-5 py-3",
          lg: "px-6 py-4 text-lg"
        }
      },
      defaultVariants: {
        variant: "primary",
        size: "md"
      }
    }
  );

  interface Props extends VariantProps<typeof buttonVariants> {
    onclick?: () => void;
    disabled?: boolean;
    type?: 'button' | 'submit' | 'reset';
    class?: string;
    children: any;
  }

  let { 
    onclick, 
    disabled = false, 
    type = 'button', 
    variant = 'primary',
    size = 'md',
    class: className = '',
    children 
  }: Props = $props();
</script>

<button
  class={buttonVariants({ variant, size, class: className })}
  {onclick}
  {disabled}
  {type}
>
  {@render children()}
</button> 