# css/css-animations/animate-with-background-color-oklch-002.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animate-with-background-color-oklch-002.html"
}
```

## style[0]

```css

  @keyframes bg-color-oklch {
    to {
      background-color: oklch(45% 0.2 264); /*  blue */
    }
  }
  @keyframes bg-mix-color-oklch {
    to {
      background-color: color-mix(in oklch, oklch(45% 0.2 264), oklch(45% 0.2 264));
    }
  }

  #target {
    background: #ff0000;
    animation-duration: 1s;
    animation-timing-function: linear;
    animation-play-state: paused;
    animation-fill-mode: forwards;
    height: 100px;
    width: 100px;
  }
  .bg-color-oklch {
    animation-name: bg-color-oklch;
  }
  .bg-mix-color-oklch {
    animation-name: bg-mix-color-oklch;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
