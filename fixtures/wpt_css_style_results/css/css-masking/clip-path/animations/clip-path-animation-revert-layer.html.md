# css/css-masking/clip-path/animations/clip-path-animation-revert-layer.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/clip-path-animation-revert-layer.html"
}
```

## style[0]

```css

  @keyframes clippath {
    0% {
      clip-path: inset(25% 25%);
    }

    100% {
      clip-path: revert-layer;
    }
  }

  .target {
    animation: clippath 20s steps(2, jump-end) -9.999s;
    background-color: blue;
    width: 100px;
    height: 100px;
  }

  /*
   * Ensure that clip path is truly none, and not a rectangle that
   * encompasses area of the parent
   */
  .outofbounds {
    position: absolute;
    top: 200px;
    left: 200px;
    height: 10px;
    width: 10px;
    background-color: blue
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
