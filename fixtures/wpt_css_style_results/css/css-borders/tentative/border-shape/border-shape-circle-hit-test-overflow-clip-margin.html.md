# css/css-borders/tentative/border-shape/border-shape-circle-hit-test-overflow-clip-margin.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/tentative/border-shape/border-shape-circle-hit-test-overflow-clip-margin.html"
}
```

## style[0]

```css

  #outer {
    width: 100px;
    height: 100px;
    overflow: clip;
    overflow-clip-margin: 50px;
  }
  #target {
    width: 100px;
    height: 100px;
    border-shape: circle(45px at 50% 50%);
    border: 10px solid purple;
    background: green;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "“overflow-clip-margin” is not supported by Safari.",
      "severity": "Warning"
    },
    {
      "message": "Unknown property “border-shape”.",
      "severity": "Error"
    }
  ],
  "warnings": 1
}
```
