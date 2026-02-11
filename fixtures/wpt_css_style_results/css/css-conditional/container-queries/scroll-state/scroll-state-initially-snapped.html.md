# css/css-conditional/container-queries/scroll-state/scroll-state-initially-snapped.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/scroll-state/scroll-state-initially-snapped.html"
}
```

## style[0]

```css

  :root {
    scroll-snap-type: block mandatory;
  }
  body {
    margin: 0;
  }
  #filler-before {
    height: 200px;
  }
  #filler-after {
    height: 10000px;
  }
  #snapped {
    container-name: initially-snapped;
    container-type: scroll-state;
    scroll-snap-align: start;
    width: 100px;
    height: 100px;
    background: teal;
  }

  @container initially-snapped scroll-state(snapped: block) {
    span { --snapped: yes }
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
