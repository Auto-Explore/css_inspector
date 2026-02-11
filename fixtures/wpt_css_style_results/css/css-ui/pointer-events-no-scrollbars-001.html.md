# css/css-ui/pointer-events-no-scrollbars-001.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/pointer-events-no-scrollbars-001.html"
}
```

## style[0]

```css


#scroll {
  width: 200px;
  height: 200px;
  overflow: auto;
  pointer-events: none;
}

#big {
  width: 500px;
  height: 500px;
}

```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “pointer-events”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
