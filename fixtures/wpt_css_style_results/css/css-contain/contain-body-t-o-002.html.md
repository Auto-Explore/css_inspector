# css/css-contain/contain-body-t-o-002.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-body-t-o-002.html"
}
```

## style[0]

```css

html {
    writing-mode: vertical-lr;
    direction: rtl;
    width: 100vw;
    height: 100vh;
    overflow: hidden;
}
body {
    margin: 0;
    border-top: 100px solid red;
    border-bottom: 100px solid green;
    width: 100vw;
    height: 100vh;
    text-orientation: upright;
    contain: paint;
}
p {
    margin: auto;
    padding: 150px 0;
    writing-mode: horizontal-tb;
    direction: ltr;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
