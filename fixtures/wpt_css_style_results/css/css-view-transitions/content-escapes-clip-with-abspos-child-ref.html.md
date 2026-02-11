# css/css-view-transitions/content-escapes-clip-with-abspos-child-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/content-escapes-clip-with-abspos-child-ref.html"
}
```

## style[0]

```css

#target {
  width: 100px;
  height: 100px;
  background: lightblue;
}
#abspos {
  position: absolute;
  left: 0;
  width: 50px;
  height: 50px;
  background: pink;
}

body { background: rebeccapurple }
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
