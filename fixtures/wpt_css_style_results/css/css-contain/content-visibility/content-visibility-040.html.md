# css/css-contain/content-visibility/content-visibility-040.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-040.html"
}
```

## style[0]

```css

#container {
  width: 150px;
  height: 150px;
  background: lightblue;
  position: relative;
}
div > div {
  width: 100px;
  height: 100px;
  background: red;
}
.hidden {
  content-visibility: hidden;
}
.flex { display: flex; }
.abspos { position: absolute; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
