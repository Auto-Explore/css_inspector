# css/css-variables/variable-declaration-51.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-51.html"
}
```

## style[0]

```css

body {
  color: orange;
  --c: var(--a);
}
p {
  --a: var(--b);
}
p {
  color: red;
  --b: var(--c,green);
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
