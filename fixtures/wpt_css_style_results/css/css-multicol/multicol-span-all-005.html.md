# css/css-multicol/multicol-span-all-005.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-span-all-005.html"
}
```

## style[0]

```css

  .column {
    column-count: 3;
    column-rule: 6px solid;
    width: 400px;
    outline: 1px solid black;
  }
  .spanner {
    column-span: all;
    outline: 1px solid blue;
  }
  fieldset {
    margin: 0;
    padding: 0;
    border: 0;
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
