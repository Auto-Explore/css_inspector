# css/css-variables/variable-declaration-22.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-22.html"
}
```

## style[0]

```css

p {
  color: red;
}
p {
  color: orange;
  --a: var(--b, var(--c, var(--d, green)));
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
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
