# css/css-borders/tentative/border-shape/border-shape-circle-hit-test-overflow-clip.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/tentative/border-shape/border-shape-circle-hit-test-overflow-clip.html"
}
```

## style[0]

```css

  #outer {
    width: 100px;
    height: 100px;
    overflow: clip;
  }
  #target {
    width: 100px;
    height: 100px;
    border-shape: circle(50% at 50% 50%);
    stroke: purple;
    stroke-width: 10px;
    background: green;
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “border-shape”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “stroke”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “stroke-width”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
