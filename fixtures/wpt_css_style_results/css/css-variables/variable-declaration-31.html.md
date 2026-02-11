# css/css-variables/variable-declaration-31.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-31.html"
}
```

## style[0]

```css

p {
  color: red;
}
p {
  color: orange;
  --0: green;
  color: var(--\30);
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
