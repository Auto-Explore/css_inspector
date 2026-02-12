# css/css-values/tree-counting/calc-sibling-function.html

```json
{
  "format_version": 3,
  "file": "css/css-values/tree-counting/calc-sibling-function.html"
}
```

## style[0]

```css

    html {
      z-index: calc(sibling-index() * 2);
      widows: calc(sibling-count() * 2);
    }
    #test {
      z-index: calc(sibling-index());
      counter-increment: foo calc(sibling-count());
      left: calc(10% + 100px * sibling-index());
    }
    #test::before {
      content: "";
      z-index: calc(sibling-index() * 2);
      widows: calc(sibling-count() * 2);
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
