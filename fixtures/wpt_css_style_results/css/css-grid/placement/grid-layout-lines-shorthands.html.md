# css/css-grid/placement/grid-layout-lines-shorthands.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/placement/grid-layout-lines-shorthands.html"
}
```

## style[0]

```css

      body {
        margin: 0;
        padding: 0;
      }
      #caseTitle {
        margin: 10px;
        height: 40px;
      }
      #grid {
        width: 150px;
        background: #eee;
        display: grid;
        grid-template-columns: [left] 100px [center] 50px [right];
      }
      .a {
        background: blue;
        grid-column: left / center;
      }
      .b {
        background: yellow;
        grid-column: center / right;
      }
    
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
