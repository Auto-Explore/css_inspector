# css/css-multicol/multicol-rule-nested-balancing-002.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-rule-nested-balancing-002.html"
}
```

## style[0]

```css

  .outer {
    column-count: 2;
    column-rule: 6px solid black;
    width: 400px;
    height: 250px;
  }
  .inner {
    column-count: 2;
    column-rule: 3px solid gray;
    height: auto;
  }
  .outer-block {
    background-color: lightgreen;
    height: 200px;
  }
  .inner-block {
    background-color: lightblue;
    height: 400px;
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
