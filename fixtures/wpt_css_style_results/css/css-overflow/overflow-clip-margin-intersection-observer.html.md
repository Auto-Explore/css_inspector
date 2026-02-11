# css/css-overflow/overflow-clip-margin-intersection-observer.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/overflow-clip-margin-intersection-observer.html"
}
```

## style[0]

```css

  #clipped_container {
    overflow: clip;
    width: 100px;
    height: 100px;
    border: solid;
    overflow-clip-margin: 50px;
  }
  #big_green_div {
      position: relative;
      width: 1000px;
      height: 1000px;
      background: green;
      left: -200px;
      top: -200px;
  }
  /* These values ensure the element is vertically offscreen. */
  .spacer { width: 150px; height: calc(100vh + 10px); }
```

```json
{
  "errors": 0,
  "messages": [
    {
      "message": "“overflow-clip-margin” is not supported by Safari.",
      "severity": "Warning"
    }
  ],
  "warnings": 1
}
```
