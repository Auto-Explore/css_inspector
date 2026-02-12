# css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-intrinsic-sizing-rows-004-auto-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-intrinsic-sizing-rows-004-auto-ref.html"
}
```

## style[0]

```css


@import "support/grid-lanes-intrinsic-sizing-visual.css";

grid {
  display: inline-grid;
  gap: 1px 2px;
  grid-template-rows: repeat(4, auto);
  grid-auto-flow: column;
  border: 1px solid;
  padding: 1px 0 2px 0;
  vertical-align: top;
  height: max-content;
  font-family: Ahem;
}

item {
  justify-self: start;
  writing-mode: vertical-rl;
  text-orientation: sideways;
}

grid > item:nth-child(1) {
  background-color: #89CFF0;
}

grid > item:nth-child(2) {
  background-color: #FF6F61;
}

grid > item:nth-child(3) {
  background-color: #FDF3E7;
}

grid > item:nth-child(4) {
  background-color: #F4C542;
}

grid > item:nth-child(5) {
  background-color: #333333;
}

grid > item:nth-child(6) {
  background-color: #B2C8A5;
}

.hidden {
  visibility: hidden;
  opacity: 0.5;
  width: 1em;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
