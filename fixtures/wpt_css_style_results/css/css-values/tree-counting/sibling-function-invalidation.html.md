# css/css-values/tree-counting/sibling-function-invalidation.html

```json
{
  "format_version": 3,
  "file": "css/css-values/tree-counting/sibling-function-invalidation.html"
}
```

## style[0]

```css

  #t1 {
    width: calc(10px * sibling-index());
    height: 50px;
    background: teal;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

  #t2 {
    width: 50px;
    height: calc(10px * sibling-count());
    background: teal;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
