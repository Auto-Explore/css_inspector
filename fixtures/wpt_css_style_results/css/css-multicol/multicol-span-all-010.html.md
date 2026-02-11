# css/css-multicol/multicol-span-all-010.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-span-all-010.html"
}
```

## style[0]

```css

  article {
    column-count: 3;
    column-rule: 6px solid;
    width: 500px;
    border: 1px solid black;
    margin-bottom: 3px;
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
