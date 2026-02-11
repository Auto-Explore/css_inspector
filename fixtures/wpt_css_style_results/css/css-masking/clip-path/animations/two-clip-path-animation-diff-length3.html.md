# css/css-masking/clip-path/animations/two-clip-path-animation-diff-length3.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/two-clip-path-animation-diff-length3.html"
}
```

## style[0]

```css

  .container {
    width: 100px;
    height: 100px;
    background-color: green;
    animation: clippath2 10000s step-end, clippath1 20s 0.001s;
  }

  /* Use un-interpolatable keyframes to force discrete transition */
  @keyframes clippath1 {
    0% {
      clip-path: circle(50% at 50% 50%);
    }

    100% {
      clip-path: inset(11px 11px);
    }
  }

  @keyframes clippath2 {
    0% {
      clip-path: circle(10% at 50% 50%);
    }

    100% {
      clip-path: circle(25% at 50% 50%);
    }
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
