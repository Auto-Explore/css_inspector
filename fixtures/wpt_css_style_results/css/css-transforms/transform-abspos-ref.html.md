# css/css-transforms/transform-abspos-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-abspos-ref.html"
}
```

## style[0]

```css

      body > div {
        width: 100px;
        height: 200px;
        position: relative;
        left: 50px;
        top: 50px;
        background: gold;
      }
      body > div > div {
        width: 200px;
        height: 100px;
        position: absolute;
        left: 50px;
        top: 100px;
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
