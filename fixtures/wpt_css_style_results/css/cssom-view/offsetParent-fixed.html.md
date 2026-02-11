# css/cssom-view/offsetParent-fixed.html

```json
{
  "format_version": 3,
  "file": "css/cssom-view/offsetParent-fixed.html"
}
```

## style[0]

```css

#a, #b, #c, #d, #e, #f {
  position: fixed;
}
#margin {
  margin: 10px;
}
#transform {
  transform: translateX(10px);
}
#perspective {
  perspective: 10px;
}
#filter {
  filter: opacity(25%);
}
#contain {
  contain: paint;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
