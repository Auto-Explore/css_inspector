# css/css-masking/clip-path/animations/clip-path-animation-custom-timing-function.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/clip-path-animation-custom-timing-function.html"
}
```

## style[0]

```css

  @keyframes clippath {
    0% {
      clip-path: inset(45% 45%);
    }

    25% {
      clip-path: inset(40% 40%);
    }

    50% {
      clip-path: inset(45% 45%);
    }

    75% {
      clip-path: inset(40% 40%);
    }

    100% {
      clip-path: inset(45% 45%);
    }
  }

  .green {
    background-color: green;
    width: 200px;
    height: 200px;
    animation: clippath 10000000s -1286796s
      /* roughly where the derivative of the cubic-bezier is zero */
    ;
    animation-timing-function: cubic-bezier(0, 9, 1, 9);
  }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
