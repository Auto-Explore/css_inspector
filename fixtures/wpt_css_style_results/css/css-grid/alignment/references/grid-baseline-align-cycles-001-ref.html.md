# css/css-grid/alignment/references/grid-baseline-align-cycles-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/references/grid-baseline-align-cycles-001-ref.html"
}
```

## style[0]

```css

  .grid {
    border: solid silver;
    margin: 1em 0.25em;
    display: inline-grid;
    grid-template-columns: repeat(3, auto);
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
  .percent, .orthogonal {
    height: 1em;
  }

  .self > div {
    align-self: start;
  }
  .content > div {
    align-content: start;
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
