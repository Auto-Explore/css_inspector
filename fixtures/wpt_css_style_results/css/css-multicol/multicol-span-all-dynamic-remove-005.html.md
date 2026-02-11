# css/css-multicol/multicol-span-all-dynamic-remove-005.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-span-all-dynamic-remove-005.html"
}
```

## style[0]

```css

  #column {
    column-count: 3;
    column-rule: 6px solid;
    width: 400px;
    outline: 1px solid black;
  }
  #block {
    height: 400px;
  }
  h3 {
    column-span: all;
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
