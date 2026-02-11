# css/css-masking/clip-path/animations/clip-path-animation-fragmented.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/clip-path-animation-fragmented.html"
}
```

## style[0]

```css

  @keyframes clippath {
    0% {
      clip-path: circle(5% at 50% 50%);
    }

    100% {
      clip-path: circle(45% at 50% 50%);
    }
  }

  .outer {
    columns: 2;
    width: 200px;
    height: 100px;
  }

  .inner {
    background-color: blue;
    animation: clippath 20s steps(2, jump-end) -9.999s;
    height: 100px;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
