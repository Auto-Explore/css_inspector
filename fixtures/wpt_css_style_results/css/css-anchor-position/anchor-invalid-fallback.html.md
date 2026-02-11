# css/css-anchor-position/anchor-invalid-fallback.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-invalid-fallback.html"
}
```

## style[0]

```css

  :root {
    --top: top;
  }
  #cb {
    position: relative;
    width: 200px;
    height: 200px;
    border: 1px solid black;
  }

  #anchor {
    anchor-name: --a;
    position: absolute;
    width: 50px;
    height: 40px;
    left: 75px;
    top: 75px;
    background: coral;
  }

  #main > div, #ref {
    position: absolute;
    background: seagreen;
  }

  #ref {
    inset: unset;
    width: unset;
    height: unset;
    min-width: unset;
    min-height: unset;
    max-width: unset;
    max-height: unset;
  }

```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
