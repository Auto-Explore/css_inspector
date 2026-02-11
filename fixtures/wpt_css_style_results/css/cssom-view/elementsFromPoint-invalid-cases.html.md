# css/cssom-view/elementsFromPoint-invalid-cases.html

```json
{
  "format_version": 3,
  "file": "css/cssom-view/elementsFromPoint-invalid-cases.html"
}
```

## style[0]

```css

html {
    overflow-y: scroll;
    overflow-x: scroll;
}
html, body {
    margin: 0;
    padding: 0;
}
body {
    width: 100%;
    height: 100%;
}
#simpleDiv {
    width: 200px;
    height: 200px;
    background-color: rgba(0,255,0,0.5);
}
#beyondTopLeft {
    position: absolute;
    transform: translate3d(-100px, -100px, 10px);
    left: 0;
    top: 0;
    width: 100px;
    height: 100px;
    background-color: rgba(0,0,0,0.1);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
