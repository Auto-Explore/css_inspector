# css/css-values/calc-invalid-range-clamping.html

```json
{
  "format_version": 3,
  "file": "css/css-values/calc-invalid-range-clamping.html"
}
```

## style[0]

```css


      html, body { margin: 0px; padding: 0px; }

      html { background: white; overflow: hidden; }
      #outer { background: green; width: 200px; height: 200px; }
      #outer { border-radius: 10px; border-radius: calc(-10px); }

  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
