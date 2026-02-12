# css/css-masking/clip-path/animations/clip-path-animation-initial.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/clip-path-animation-initial.html"
}
```

## style[0]

```css

  @keyframes clippath {
    0% {
      clip-path: inset(25% 25%);
    }

    100% {
      clip-path: initial;
    }
  }

  .target {
    animation: clippath 20s steps(2, jump-end) -9.999s;
    background-color: blue;
    width: 100px;
    height: 100px;
  }

  /*
   * Ensure that clip-path is truly none, and not a rectangle that
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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
