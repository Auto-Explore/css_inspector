# css/printing/pseudo-animations-print.html

```json
{
  "format_version": 3,
  "file": "css/printing/pseudo-animations-print.html"
}
```

## style[0]

```css


@keyframes a {
  from, to { color: blue }
}

p {
  color: olive;
  animation: a 1s infinite;
}
p::after {
    color: purple;
    content: "(::after)";
    animation: a 1s infinite;
}

p::before {
    color: yellow;
    content: "(::before)";
    animation: a 1s infinite;
}

```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
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
