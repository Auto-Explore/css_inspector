# css/css-properties-values-api/registered-property-computation-color-002.html

```json
{
  "format_version": 3,
  "file": "css/css-properties-values-api/registered-property-computation-color-002.html"
}
```

## style[0]

```css

@property --x {
  inherits: true;
  initial-value: black;
  syntax: "<color>";
}
div {
  color-scheme: dark;
  --x: light-dark(red, green);
  --y: var(--x);
  background-color: var(--y);
  width: 100px;
  height: 100px;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
