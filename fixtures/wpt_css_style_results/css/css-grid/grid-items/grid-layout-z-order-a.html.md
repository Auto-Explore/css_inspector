# css/css-grid/grid-items/grid-layout-z-order-a.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-layout-z-order-a.html"
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
        grid-template-columns: 50px 50px 50px;
        color:white;
      }
      .a {
        background: blue;
        grid-row: 1;
        grid-column: 1 / span 2;
        z-index:10
      }
      .b {
        background: yellow;
        grid-row: 1;
        grid-column: 2 / span 2;
        z-index:1
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
