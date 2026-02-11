# css/css-values/viewport-units-compute.html

```json
{
  "format_version": 3,
  "file": "css/css-values/viewport-units-compute.html"
}
```

## style[0]

```css

  iframe {
    width: 200px;
    height: 100px;
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

        * { margin: 0; }
        body { height: 100%; }
        div { height: ${value}; }
      
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
