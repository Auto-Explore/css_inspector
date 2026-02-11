# css/cssom-view/long_scroll_composited.html

```json
{
  "format_version": 3,
  "file": "css/cssom-view/long_scroll_composited.html"
}
```

## style[0]

```css

.post {
  height: 1000px;
  width: 300px;
  border: 1px solid black;

}
.scroller {
  overflow-y: scroll;
  width: 500px;
  height: 500px;
  will-change: transform;
}
::-webkit-scrollbar {
  display: none;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
