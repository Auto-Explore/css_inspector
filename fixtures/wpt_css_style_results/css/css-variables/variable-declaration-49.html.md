# css/css-variables/variable-declaration-49.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-49.html"
}
```

## style[0]

```css

body {
  color: orange;
}
p {
  color: crimson;
  --a: red var(--b) var(--g);
  --b: var(--c);
  --c: var(--d);
  --d: var(--e);
  --e: var(--a);
  --f: var(--e);
  --g: green;
  color: var(--g);
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
