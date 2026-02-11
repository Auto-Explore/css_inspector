# css/css-values/tree-counting/sibling-function-container-query.html

```json
{
  "format_version": 3,
  "file": "css/css-values/tree-counting/sibling-function-container-query.html"
}
```

## style[0]

```css

  @property --length {
    syntax: "<length>";
    initial-value: 0px;
    inherits: false;
  }
  .container { container-type: inline-size; }
  #c1 {
    width: 100px;
    --length: 100px;
  }
  #c2 {
    width: 400px;
    --length: 400px;
  }
  span {
    --match-100: no;
    --match-400: no;
  }
  @container (width = calc(100px * sibling-index())) {
    span { background-color: green; }
  }
  @container (width = calc(200px * sibling-count())) {
    span { color: lime; }
  }
  @container style(--length: calc(100px * sibling-index())) {
    span { --match-100: yes; }
  }
  @container style(--length: calc(200px * sibling-count())) {
    span { --match-400: yes; }
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
