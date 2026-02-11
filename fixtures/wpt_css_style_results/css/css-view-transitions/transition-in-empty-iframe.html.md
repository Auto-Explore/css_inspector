# css/css-view-transitions/transition-in-empty-iframe.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/transition-in-empty-iframe.html"
}
```

## style[0]

```css

    iframe {
      position: absolute;
      left: 25px;
      top: 25px;
      width: 50vw;
      height: 50vh;
    }
    /* This div overlaps with the iframe, because the iframe is empty it should
     * be visible behind the iframe. */
    div {
      background-color: skyblue;
      width: 100px;
      height: 100px;
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
