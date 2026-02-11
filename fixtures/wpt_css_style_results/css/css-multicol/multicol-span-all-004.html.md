# css/css-multicol/multicol-span-all-004.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-span-all-004.html"
}
```

## style[0]

```css

  #column {
    column-count: 3;
    column-rule: 6px solid;
    width: 500px;
    outline: 1px solid black;
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
