# css/filter-effects/feComposite-intersection-feTile-input.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/feComposite-intersection-feTile-input.html"
}
```

## style[0]

```css

body {
  margin: 0;
}
div {
  position: absolute;
  top: 0;
  border: solid 20px;
  width: 100px; height: 100px;
  filter: url(#test);
}
div::after {
  opacity: 0.5;
  position: absolute;
  inset: 0;
  background: #0f0;
  content: '';
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
