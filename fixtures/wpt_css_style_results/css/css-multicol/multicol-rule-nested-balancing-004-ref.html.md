# css/css-multicol/multicol-rule-nested-balancing-004-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-rule-nested-balancing-004-ref.html"
}
```

## style[0]

```css

  .outer {
    column-count: 1;
    column-rule: 6px solid black;
    column-fill: auto;
    background-color: rgba(0, 0, 255, 0.3);
    width: 200px;
    height: 300px;
  }
  .inner {
    column-count: 1;
    column-rule: 3px solid gray;
    column-fill: auto;
    background-color: rgba(255, 0, 255, 0.3);
  }
  .inner-block {
    background-color: rgba(0, 255, 0, 0.3);
  }
  .space {
    height: 100px;
  }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
