# css/css-grid/reference/grid-layout-grid-in-grid-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/reference/grid-layout-grid-in-grid-ref.html"
}
```

## style[0]

```css

      body {
        margin: 0;
        padding: 0;
        border: 0 none;
      }
      #caseTitle {
        margin: 10px;
        height: 40px;
      }
      #grid {
        margin: 0;
        height: 150px;
        width: 150px;
        background: yellow;
        position: relative;
      }
      .a {
        background: blue;
        height: 100px;
        width: 50px;
        position: absolute;
        top: 0;
        left: 0;
      }
      .b1 {
        background: orange;
        width: 50px;
        height: 50px;
        position: absolute;
        top: 0;
        left: 50px;
      }
      .b2 {
        background: cyan;
        width: 50px;
        height: 50px;
        position: absolute;
        top: 50px;
        left: 100px;
      }
      .c {
        background: pink;
        width: 100px;
        height: 50px;
        position: absolute;
        top: 100px;
        left: 50px;
      }
      .d {
        background: #eee;
        width: 50px;
        height: 50px;
        position: absolute;
        top: 100px;
        left: 0
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
