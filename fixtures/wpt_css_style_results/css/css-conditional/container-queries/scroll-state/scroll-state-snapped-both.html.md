# css/css-conditional/container-queries/scroll-state/scroll-state-snapped-both.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/scroll-state/scroll-state-snapped-both.html"
}
```

## style[0]

```css

  :root {
    scroll-snap-type: both mandatory;
  }
  body {
    margin: 0;
  }
  #filler {
    width: 10000px;
    height: 10000px;
  }
  #snapped {
    margin-top: 200px;
    margin-left: 200px;
    width: 100px;
    height: 100px;
    container-type: scroll-state;
    scroll-snap-align: start;
    background: teal;
  }

  @container scroll-state(snapped: both) {
    #target { --snapped: yes }
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
