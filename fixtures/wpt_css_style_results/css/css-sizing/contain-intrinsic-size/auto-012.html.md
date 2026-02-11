# css/css-sizing/contain-intrinsic-size/auto-012.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/contain-intrinsic-size/auto-012.html"
}
```

## style[0]

```css

#target {
  content-visibility: auto;
  contain-intrinsic-size: auto 100px auto 101px;
  width: max-content;
  height: max-content;
  border: 1px solid;
}
#target::before {
  content: "";
  display: block;
  width: 50px;
  height: 51px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “contain-intrinsic-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
