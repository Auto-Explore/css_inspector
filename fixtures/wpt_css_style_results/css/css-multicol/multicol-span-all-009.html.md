# css/css-multicol/multicol-span-all-009.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-span-all-009.html"
}
```

## style[0]

```css

  article {
    column-count: 3;
    column-rule: 6px solid;
    width: 400px;
    outline: 1px solid black;
    transform: scale(1);
  }
  h3 {
    column-span: all;
    outline: 1px solid blue;
  }
  .out-of-flow {
    position: absolute;
    top: 0;
    right: 0;
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
