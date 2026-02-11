# css/motion/offset-path-url-crash.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-url-crash.html"
}
```

## style[0]

```css

      #test {
        position: absolute;
        left: 300px;
        top: 0px;
        width: 300px;
        height: 200px;
        background-color: lime;
        transform-origin: 0px 0px;
        offset-path: url(#target);
      }
    
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “offset-path”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
