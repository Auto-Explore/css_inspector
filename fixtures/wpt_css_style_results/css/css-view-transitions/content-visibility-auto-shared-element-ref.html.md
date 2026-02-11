# css/css-view-transitions/content-visibility-auto-shared-element-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/content-visibility-auto-shared-element-ref.html"
}
```

## style[0]

```css

body { background: pink }
.flex {
  display: flex;
  flex-direction: row;
  justify-content: flex-start;
  align-items: flex-start;
}
.box {
  width: 100px;
  height: 500px;
  contain: paint;
  background: green;
  border: 1px solid black;
  box-sizing: border-box;
}
.vis-hidden {
  visibility: hidden;
}
.cv-hidden {
  content-visibility: hidden;
}
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
