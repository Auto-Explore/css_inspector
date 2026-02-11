# css/css-variables/variable-declaration-38.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-38.html"
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
  --A: crimson;
  color: var(--a);
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
