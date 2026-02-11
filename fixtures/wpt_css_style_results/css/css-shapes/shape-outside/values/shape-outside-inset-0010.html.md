# css/css-shapes/shape-outside/values/shape-outside-inset-0010.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/values/shape-outside-inset-0010.html"
}
```

## style[0]

```css

#container { width: 100px; line-height: 0; }
#float_1 { float: right; width: 30px; height: 30px; background: green; }
#float_2 { float: left; width: 20px; height: 100px; background: green; shape-outside: inset(0 20px 0 0); }
#float_3 { float: left; width: 30px; height: 50px; background: green; }
.atomic { display: inline-block; background: green; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “shape-outside”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
