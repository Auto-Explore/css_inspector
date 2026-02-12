# css/css-view-transitions/scoped/crashtests/zindex-part.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/crashtests/zindex-part.html"
}
```

## style[0]

```css


#scope {
  position: relative; width: 100px; height: 100px;
  background: #ddd; contain: strict;
}
#part {
  position: absolute; top: 10px; left: 10px;
  width: 80px; height: 80px; background: #8cf;
  z-index: 1; view-transition-name: foo;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
