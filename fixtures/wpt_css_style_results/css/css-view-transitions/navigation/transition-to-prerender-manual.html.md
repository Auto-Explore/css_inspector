# css/css-view-transitions/navigation/transition-to-prerender-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/navigation/transition-to-prerender-manual.html"
}
```

## style[0]

```css

@view-transition { navigation: auto; }
html { background: red; }
#target {
  width: 200px;
  height: 200px;
  background: black;
  color: white;
  position: absolute;
  top: 40px;
  view-transition-name: target;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
