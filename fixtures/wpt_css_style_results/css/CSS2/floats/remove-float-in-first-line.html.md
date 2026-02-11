# css/CSS2/floats/remove-float-in-first-line.html

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats/remove-float-in-first-line.html"
}
```

## style[0]

```css

#container {
  display: flow-root;
}
#container::first-line {
  background: orange;
}
#float {
  float: left;
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
