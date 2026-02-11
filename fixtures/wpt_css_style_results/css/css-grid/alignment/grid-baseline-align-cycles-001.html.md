# css/css-grid/alignment/grid-baseline-align-cycles-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-baseline-align-cycles-001.html"
}
```

## style[0]

```css

  .grid {
    border: solid silver;
    margin: 1em 0.25em;
    display: inline-grid;
    grid-template-columns: repeat(2, auto);
    font: 20px/1 Ahem;
    height: 5em;
  }
  .grid > div {
    border: 1em aqua;
    border-style: solid none;
  }
  .index {
    padding: 1em 0;
  }
  .percent {
    height: 20%;
  }
  .orthogonal {
    height: 20%;
    writing-mode: vertical-rl;
  }

  .self > div {
    align-self: baseline;
  }
  .content > div {
    align-content: baseline;
  }
  .self.ref > div {
    align-self: start;
  }
  .content.ref > div {
    align-content: start;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
