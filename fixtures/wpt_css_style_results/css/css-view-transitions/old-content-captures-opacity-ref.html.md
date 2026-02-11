# css/css-view-transitions/old-content-captures-opacity-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/old-content-captures-opacity-ref.html"
}
```

## style[0]

```css

.box {
  color: red;
  background: lightblue;
  width: 100px;
  height: 100px;
  contain: paint;
  position: absolute;
  font-size: 30pt;
  will-change: opacity;
}
#e1 { opacity: 0.75; top: 20px; left: 20px; }
#e2 { opacity: 0.5; top: 160px; left: 20px; }
#e3 { opacity: 0.25; top: 300px; left: 20px; }
body { background: lightpink; }
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
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
