# css/css-contain/contain-size-multicol-004.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-size-multicol-004.html"
}
```

## style[0]

```css

    .contain {
      contain:size;
    }
    .cols {
      column-count: 3;
      column-rule: 1px dotted blue;
      column-fill: auto;
      border: 2px solid blue;
      height: 50px;
      width: 300px;
    }
    .innerObject {
      height: 200px;
      width: 100px;
      background: orange;
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
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
