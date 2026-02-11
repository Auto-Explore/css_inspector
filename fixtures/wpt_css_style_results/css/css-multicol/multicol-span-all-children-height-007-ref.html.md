# css/css-multicol/multicol-span-all-children-height-007-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-span-all-children-height-007-ref.html"
}
```

## style[0]

```css

  .outer {
    column-count: 2;
    column-fill: auto;
    column-rule: 6px solid black;
    width: 400px;
    height: 110px;
  }
  .inner {
    column-count: 2;
    column-rule: 3px solid green;
    box-sizing: border-box;
    height: 110px;
    background-color: lightgreen;
    border: solid purple;
  }
  .block1 {
    background-color: yellow;
    height: 200px;
  }
  .spanner {
    column-span: all;
    height: 50px;
    background-color: lightblue;
  }
  .block2 {
    background-color: yellow;
    height: 120px;
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
