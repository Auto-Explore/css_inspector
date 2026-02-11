# css/css-view-transitions/scoped/scrolled-target-position-rtl-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/scrolled-target-position-rtl-ref.html"
}
```

## style[0]

```css

#target {
  width: 100px;
  height: 100px;
  overflow: scroll;
  background: lightblue;
  direction: rtl;
}
#child {
  width: 50px;
  height: 75px;
  background: green;
}
#content {
  width: 100px;
  height: 400px;
  background: yellow;
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
