# css/css-multicol/column-balancing-paged-001-print.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/column-balancing-paged-001-print.html"
}
```

## style[0]

```css

  :root {
    print-color-adjust: exact;
  }
  .outer {
    column-count: 1;
    column-rule: 6px solid black;
  }
  .inner {
    column-count: 2;
    column-rule: 3px solid green;
    background-color: lightgreen;
  }
  .block1 {
    background-color: yellow;
    height: 3in;
  }
  .spanner {
    column-span: all;
    height: 2in;
    background-color: lightblue;
  }
  .block2 {
    background-color: pink;
    height: 3in
  }
  
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
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
