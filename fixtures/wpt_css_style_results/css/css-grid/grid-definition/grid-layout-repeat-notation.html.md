# css/css-grid/grid-definition/grid-layout-repeat-notation.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-definition/grid-layout-repeat-notation.html"
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
        width: 450px;
        background: #eee;
        display: grid;
        grid-template-columns: repeat(4, 100px) 50px;
      }
      .a {
        background: blue;
        grid-column: 1;
      }
      .b {
        background: yellow;
        grid-column: 2;
      }
      .c {
        background: orange;
        grid-column: 3;
      }
      .d {
        background: cyan;
        grid-column: 4;
      }
      .e {
        background: pink;
        grid-column: 5;
      }
    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
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
