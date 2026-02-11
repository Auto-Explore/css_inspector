# css/css-variables/variable-declaration-32.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-32.html"
}
```

## style[0]

```css

p {
  color: red;
}
p {
  color: orange;
  --\30: green;
  color: var(--\30);
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
      "message": "Invalid property name in declaration.",
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
