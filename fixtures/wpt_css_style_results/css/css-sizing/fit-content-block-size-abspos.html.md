# css/css-sizing/fit-content-block-size-abspos.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/fit-content-block-size-abspos.html"
}
```

## style[0]

```css

body {
  overflow: hidden;
}
#container {
  height: fit-content;
  position: absolute;
  top: 50%;
  background-color: gold;
  margin: auto;
  bottom: 0;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
