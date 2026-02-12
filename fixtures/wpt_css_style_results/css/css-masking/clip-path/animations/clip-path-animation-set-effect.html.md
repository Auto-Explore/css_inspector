# css/css-masking/clip-path/animations/clip-path-animation-set-effect.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/clip-path-animation-set-effect.html"
}
```

## style[0]

```css

  .container {
    width: 100px;
    height: 100px;
    background-color: green;
    animation: clippath 10s steps(2, jump-end);
  }

  @keyframes clippath {
    0% {
      clip-path: inset(10% 10%);
    }

    100% {
      clip-path: inset(40% 40%);
    }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
