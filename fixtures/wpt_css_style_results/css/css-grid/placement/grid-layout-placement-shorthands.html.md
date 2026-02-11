# css/css-grid/placement/grid-layout-placement-shorthands.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/placement/grid-layout-placement-shorthands.html"
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
      }
      .a {
        background: blue;
        grid-column: 1 / span 2;
        grid-row: 1;
      }
      .b {
        background: yellow;
        grid-column: 3;
        grid-row: 1;
      }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
