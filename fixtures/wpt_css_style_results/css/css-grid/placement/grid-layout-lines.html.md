# css/css-grid/placement/grid-layout-lines.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/placement/grid-layout-lines.html"
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
        grid-column-start: left;
        grid-column-end: center;
      }
      .b {
        background: yellow;
        grid-column-start: center;
        grid-column-end: right;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
