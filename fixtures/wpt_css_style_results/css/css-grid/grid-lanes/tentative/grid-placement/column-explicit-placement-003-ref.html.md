# css/css-grid/grid-lanes/tentative/grid-placement/column-explicit-placement-003-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/grid-placement/column-explicit-placement-003-ref.html"
}
```

## style[0]

```css

.grid {
    display: grid;
    background: gray;
    grid-template-columns: auto auto auto;
    height: 200px;
    width: 500px;
    gap: 20px;
    padding: 20px;
}

.first-track {
    background: lightskyblue;
    grid-column-start: 1;
    writing-mode: vertical-rl;
    height: max-content;
}

.second-track {
    background: lightcoral;
    grid-column-start: 2;
    width: fit-content;
    height: fit-content;
}

.third-track {
    background: lightgreen;
    grid-column-start: 3;
    writing-mode: vertical-lr;
    height: max-content;
}

.flex {
    display: flex;
    flex-direction: column;
    gap: 20px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
