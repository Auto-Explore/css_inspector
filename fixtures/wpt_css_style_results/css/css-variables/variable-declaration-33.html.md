# css/css-variables/variable-declaration-33.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-33.html"
}
```

## style[0]

```css

p {
  color: red;
}
p {
  color: orange;
  --\61: green;
  color: var(--a);
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
