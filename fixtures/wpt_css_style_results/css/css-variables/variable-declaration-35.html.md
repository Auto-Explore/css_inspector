# css/css-variables/variable-declaration-35.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-35.html"
}
```

## style[0]

```css

p {
  color: red;
}
p {
  color: orange;
  --\fffd: green;
  color: var(--\fffd);
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
      "message": "Invalid property name in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
