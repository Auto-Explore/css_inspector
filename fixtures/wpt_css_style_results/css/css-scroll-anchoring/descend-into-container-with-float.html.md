# css/css-scroll-anchoring/descend-into-container-with-float.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-anchoring/descend-into-container-with-float.html"
}
```

## style[0]

```css


body { height: 4000px; }
#outer { width: 300px; }
#outer:after { content: " "; clear:both; display: table; }
#float {
  float: left; background-color: #ccc;
  height: 500px; width: 100%;
}
#inner { height: 100px; background-color: green; }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
