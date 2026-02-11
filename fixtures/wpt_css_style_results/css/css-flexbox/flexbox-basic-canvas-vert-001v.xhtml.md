# css/css-flexbox/flexbox-basic-canvas-vert-001v.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-basic-canvas-vert-001v.xhtml"
}
```

## style[0]

```css

      div.flexbox {
        height: 200px;
        background: lightgreen;
        display: flex;
        justify-content: space-between;
        flex-direction: column;
        float: left;
        margin-right: 10px;
        font: 8px monospace;
      }
      canvas {
        width: 20px;
        height: 10px;
        min-height: 0;
        border: 1px dotted green;
        writing-mode: vertical-lr;
      }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
