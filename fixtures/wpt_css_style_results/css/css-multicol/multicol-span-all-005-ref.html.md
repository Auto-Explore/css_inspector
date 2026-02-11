# css/css-multicol/multicol-span-all-005-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-span-all-005-ref.html"
}
```

## style[0]

```css

  .column {
    column-count: 1;
    column-rule: 6px solid;
    width: 400px;
    outline: 1px solid black;
  }
  .spanner {
    /* column-count: 1 makes this behave like a real spanner. */
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
