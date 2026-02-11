# css/css-variables/variable-declaration-23.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-23.html"
}
```

## style[0]

```css

p {
  color: red;
}
p {
  color: orange;
  --a: green;
  --b: crimson;
  --a: var(--b) !important !important;
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
