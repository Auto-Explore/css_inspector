# css/css-pseudo/highlight-styling-003.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-styling-003.html"
}
```

## style[0]

```css

    main {
        font-size: 7em;
        margin: 0.5em;
    }
    main::selection {
        color: white;
        -webkit-text-fill-color: yellow;
        -webkit-text-stroke-color: green;
        -webkit-text-stroke-width: 4px;
    }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “-webkit-text-fill-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-webkit-text-stroke-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-webkit-text-stroke-width”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
