# css/css-variables/variable-declaration-19.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-19.html"
}
```

## style[0]

```css

p {
  color: red;
}
p {
  color: orange;
  --a: rgb(0, var(--b), 0);
  --b: 128;
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
