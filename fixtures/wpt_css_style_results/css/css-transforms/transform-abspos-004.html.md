# css/css-transforms/transform-abspos-004.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-abspos-004.html"
}
```

## style[0]

```css

      body > div {
        width: 100px;
        height: 200px;
        transform: translate(50px, 50px);
        background: gold;
      }
      body > div > div {
        width: 200px;
        height: 100px;
        position: absolute;
        right: -150px;
        bottom: 0;
        background: navy;
        color: gold;
      }
    
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
