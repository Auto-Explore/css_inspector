# css/cssom-view/scrollIntoView-scrollMargin.html

```json
{
  "format_version": 3,
  "file": "css/cssom-view/scrollIntoView-scrollMargin.html"
}
```

## style[0]

```css

#scroller {
  width: 200px;
  height: 200px;
  overflow: scroll;
}
#content {
  width: 500px;
  height: 500px;
}
#target {
  position: relative;
  left: 200px;
  top: 200px;
  width: 100px;
  height: 100px;
  scroll-margin-top: 4px;
  scroll-margin-right: 8px;
  scroll-margin-bottom: 12px;
  scroll-margin-left: 16px;
  background-color: lightgreen;
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
