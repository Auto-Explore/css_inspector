# css/css-multicol/multicol-span-all-008-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-span-all-008-ref.html"
}
```

## style[0]

```css

  body {
    column-count: 1;
    column-rule: 6px solid;
    width: 400px;
    outline: 1px solid black;
  }
  h3 {
    /* "column-count: 1" makes this behave like a real spanner. */
    outline: 1px solid blue;
  }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
