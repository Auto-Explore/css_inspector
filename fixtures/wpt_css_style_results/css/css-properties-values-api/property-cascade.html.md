# css/css-properties-values-api/property-cascade.html

```json
{
  "format_version": 3,
  "file": "css/css-properties-values-api/property-cascade.html"
}
```

## style[0]

```css


#outer { color: rgb(1, 1, 1); }
#inner {
    --my-color: rgb(2, 2, 2);
    --my-color: url(not-a-color);
    color: var(--my-color);
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
