# css/css-multicol/multicol-rule-nested-balancing-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-rule-nested-balancing-002-ref.html"
}
```

## style[0]

```css

  .outer {
    column-count: 2;
    column-rule: 6px solid black;
    column-fill: auto;
    width: 400px;
    height: 250px;
  }
  .inner {
    column-count: 2;
    column-rule: 3px solid gray;
    column-fill: auto;
    height: 200px;
  }
  .outer-block {
    background-color: lightgreen;
    height: 200px;
  }
  .inner-block {
    background-color: lightblue;
    height: 200px;
  }
  .space {
    height: 50px;
  }
  
```

```json
{
  "errors": 4,
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
    }
  ],
  "warnings": 0
}
```
