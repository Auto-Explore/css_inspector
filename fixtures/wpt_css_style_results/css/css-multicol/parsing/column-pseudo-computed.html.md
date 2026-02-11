# css/css-multicol/parsing/column-pseudo-computed.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/parsing/column-pseudo-computed.html"
}
```

## style[0]

```css

  #target {
    font-size: 20px;
    scroll-margin-bottom: 4px;
  }
  #target::column {
    font-size: 40px;
    scroll-snap-align: start;
    scroll-snap-stop: always;
    scroll-margin: 2px;
    scroll-margin-top: 2em;
    scroll-margin-bottom: inherit;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
