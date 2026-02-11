# css/css-view-transitions/scoped/target-in-scrolled-container-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/target-in-scrolled-container-ref.html"
}
```

## style[0]

```css

#scroller {
  width: 200px;
  height: 200px;
  overflow: scroll;
  background: lightgray;
}
#target {
  width: 50px;
  height: 50px;
  background: green;
  margin-top: 100px;
  view-transition-name: target;
}
#spacer {
  height: 200px;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
