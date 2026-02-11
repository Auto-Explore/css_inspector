# css/css-contain/contain-subgrid-001.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-subgrid-001.html"
}
```

## style[0]

```css

.grid {
  display: grid;
  width: 40px;
  height: 40px;
  grid: [a] repeat(2,15px) [b] / [a] repeat(2,15px) [b];
  gap: 10px;
}

.subgrid {
  display: grid;
  grid: subgrid;
  background: lightgrey;
  grid-area:1/1/3/3;
}

.layout { contain: layout; }
.paint { contain: paint; }

.inner {
  background: blue;
  grid-area:a/a/b/b;
}
.p {
  place-self: start;
  width:100%;
  height:100%;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
