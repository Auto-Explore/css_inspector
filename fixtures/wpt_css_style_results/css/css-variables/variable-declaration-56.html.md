# css/css-variables/variable-declaration-56.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-56.html"
}
```

## style[0]

```css

p {
  color: red;
}
p {
  color: orange;
  --a: unset;
  color: var(--a,green);
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
