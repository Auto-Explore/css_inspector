# css/css-values/resolution-with-percentage-without-context.html

```json
{
  "format_version": 3,
  "file": "css/css-values/resolution-with-percentage-without-context.html"
}
```

## style[0]

```css

div {
  background-color: green;
  width: 100px;
  height: 100px;
}

@media (resolution <= calc(sign(50%) * 1dpi)) {
  div {
    background-color: red;
  }
}

@media (resolution >= calc(sign(50%) * 1dpi)) {
  div {
    background-color: red;
  }
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
