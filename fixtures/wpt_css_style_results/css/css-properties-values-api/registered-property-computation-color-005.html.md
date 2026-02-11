# css/css-properties-values-api/registered-property-computation-color-005.html

```json
{
  "format_version": 3,
  "file": "css/css-properties-values-api/registered-property-computation-color-005.html"
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
  --x: color(srgb 0 calc(0.5 * sibling-index()) 0);
  background-color: var(--x);
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
