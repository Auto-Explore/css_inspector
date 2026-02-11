# css/css-values/calc-in-media-queries-002.html

```json
{
  "format_version": 3,
  "file": "css/css-values/calc-in-media-queries-002.html"
}
```

## style[0]

```css


    html { background: red; }
    @media (min-width: calc(-100px)) { /* should clamp to 0px */
      html { background: green; }
    }

  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
