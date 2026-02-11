# css/css-position/position-absolute-crash-chrome-011.html

```json
{
  "format_version": 3,
  "file": "css/css-position/position-absolute-crash-chrome-011.html"
}
```

## style[0]

```css

body {
  margin: 20px;
}
.container {
  position: relative;
}
#inline-container-absolute {
  position: relative;
  background: rgba(0,255,0,0.3);
}
#inline-container-fixed {
  filter:  blur(2px);
  background: rgba(0,255,0,0.3);
}
.outofflow {
  position: absolute;
  width: 20px;
  height: 20px;
  background: green;
  top:0;
  left:0;
}
.splitter {
  width: 100px;
  height: 20px;
  background: gray;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
