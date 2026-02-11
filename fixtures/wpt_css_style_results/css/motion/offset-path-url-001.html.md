# css/motion/offset-path-url-001.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-url-001.html"
}
```

## style[0]

```css

      #outer {
        width: 400px;
        height: 200px;
      }
      #target {
        width: 50px;
        height: 50px;
        background-color: green;
        offset-path: url(#svgPath);
        offset-anchor: 0% 0%;
        border-radius: 50% 50% 0 0;
      }
    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “offset-path”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “offset-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-radius”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
