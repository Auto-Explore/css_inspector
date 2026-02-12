# css/css-masking/clip-path/animations/two-clip-path-animation-diff-length2.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/two-clip-path-animation-diff-length2.html"
}
```

## style[0]

```css

  .container {
    width: 100px;
    height: 100px;
    background-color: green;
    animation: clippath2 100s step-end, clippath1 20s 30s;
  }

  @keyframes clippath1 {
    0% {
      clip-path: circle(10% at 50% 50%);
    }

    100% {
      clip-path: circle(50% at 50% 50%);
    }
  }

  @keyframes clippath2 {
    0% {
      clip-path: inset(10px 10px);
    }

    100% {
      clip-path: inset(11px 11px);
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
