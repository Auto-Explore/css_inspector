# css/css-variables/variable-declaration-26.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-26.html"
}
```

## style[0]

```css

p {
  color: red;
}
p {
  color: orange;
  --a: ;
  color: var(--a) green;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Missing value for property “--a”.",
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
