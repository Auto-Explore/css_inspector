# css/css-transforms/scale-animation-with-var-001.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/scale-animation-with-var-001.html"
}
```

## style[0]

```css


@keyframes k {
  to {
    scale: var(--scale-to);
  }
}

#target {
  --percentage: 3;
  --scale-to: calc(var(--percentage));

  animation: k linear 2s -1s paused;
  width: 50px;
  height: 50px;
  background: green;
  transform-origin: top left;
}

```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
