# css/css-properties-values-api/at-property-viewport-units-dynamic.html

```json
{
  "format_version": 3,
  "file": "css/css-properties-values-api/at-property-viewport-units-dynamic.html"
}
```

## style[0]

```css

  iframe {
    width: 400px;
    height: 200px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

    @property --10vw { syntax: '<length>'; inherits: true; initial-value: 10vw}
    @property --10vh { syntax: '<length>'; inherits: true; initial-value: 10vh}
    div {
      background: green;
      width: var(--10vw);
      height: var(--10vh);
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
