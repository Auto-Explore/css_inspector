# css/css-grid/grid-definition/grid-layout-auto-tracks.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-definition/grid-layout-auto-tracks.html"
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
        height: auto;
        width: 100px;
        background: #eee;
        display: grid;
        grid-template-columns: 100px;
        grid-template-rows: auto;
      }
      .a {
        background: blue;
        grid-column: 1;
        grid-row: 1;
      }
      .b {
        background: yellow;
        grid-column: 2;
        grid-row: 1;
      }
      .c {
        background: pink;
        grid-column: 1;
        grid-row: 2;
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
