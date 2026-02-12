# css/css-conditional/container-queries/scroll-state/scroll-state-snapped-container-type-change.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/scroll-state/scroll-state-snapped-container-type-change.html"
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
    container-type: scroll-state;
    scroll-snap-align: start;
    width: 100px;
    height: 100px;
    background: teal;
  }
  span {
    --snapped: no;
    @container scroll-state(snapped) {
      --snapped: yes;
    }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
