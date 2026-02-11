# css/css-values/tree-counting/sibling-function-media-query.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-values/tree-counting/sibling-function-media-query.tentative.html"
}
```

## style[0]

```css

  #target {
    background-color: green;
  }
  @media (width > CALC(0px * sibling-index())) {
    #target { background-color: red; }
  }
  @media (width > CALC(0px * sibling-count())) {
    #target { background-color: red; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
