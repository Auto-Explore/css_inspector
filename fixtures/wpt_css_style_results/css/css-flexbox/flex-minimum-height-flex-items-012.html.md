# css/css-flexbox/flex-minimum-height-flex-items-012.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flex-minimum-height-flex-items-012.html"
}
```

## style[0]

```css

.flexbox {
    display: flex;
}

.column {
    flex-flow: column;
}

.flexbox span {
    height: 100%;
    background: orange;
    display: block;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
