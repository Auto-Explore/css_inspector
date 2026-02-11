# css/css-variables/variable-declaration-21.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-21.html"
}
```

## style[0]

```css

p {
  color: red;
}
p {
  color: orange;
  --a: rgb(var(--b)var(--c)var(--d));
  --b: 0,;
  --c: 128,;
  --d: 0;
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
