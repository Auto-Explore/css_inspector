# css/css-multicol/multicol-span-all-rule-001.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-span-all-rule-001.html"
}
```

## style[0]

```css

  article {
    column-count: 2;
    column-rule: 6px solid;
    width: 400px;
    height: 500px;
    background-color: lightgreen;
    border: 2em solid purple;
    padding: 2em;
  }
  div.block {
    width: 100px;
    height: 200px;
  }
  div.column-span {
    column-span: all;
    height: 50px;
    background-color: lightblue;
  }
  
```

```json
{
  "errors": 3,
  "messages": [
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
