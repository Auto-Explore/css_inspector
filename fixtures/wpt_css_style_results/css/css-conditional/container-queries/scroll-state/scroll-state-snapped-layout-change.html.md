# css/css-conditional/container-queries/scroll-state/scroll-state-snapped-layout-change.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/scroll-state/scroll-state-snapped-layout-change.html"
}
```

## style[0]

```css

  :root {
    scroll-snap-type: block proximity;
  }
  body {
    margin: 0;
  }
  #filler-before {
    height: 10px;
  }
  #filler-after {
    height: 10000px;
  }
  #snapped {
    position: relative;
    top: 3000px; /* Should be enough to not snap for proximity */
    container-type: scroll-state;
    scroll-snap-align: start;
    width: 100px;
    height: 100px;
  }
  #target {
    --snapped: no;
    @container scroll-state(snapped) {
      --snapped: yes;
    }
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “scroll-snap-type”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
