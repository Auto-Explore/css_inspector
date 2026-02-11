# css/css-variables/variable-declaration-39.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-39.html"
}
```

## style[0]

```css

p {
  color: red;
}
p {
  color: orange;
  --A: green;
  --a: crimson;
  color: var(--A);
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
