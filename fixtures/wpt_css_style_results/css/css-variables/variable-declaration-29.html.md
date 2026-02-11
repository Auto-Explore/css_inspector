# css/css-variables/variable-declaration-29.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-29.html"
}
```

## style[0]

```css

p {
  color: green;
  --: red;
  color: var(--,crimson);
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
