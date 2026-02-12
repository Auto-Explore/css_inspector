# css/selectors/popover-open-with-has-sibling-selector.html

```json
{
  "format_version": 3,
  "file": "css/selectors/popover-open-with-has-sibling-selector.html"
}
```

## style[0]

```css

    .half-square {
        width: 100px;
        height: 50px;
        background-color: red;
    }
    #p1-sibling:not(:has(+:popover-open)) {
        background-color: green;
    }
    #p2-sibling:has(+:popover-open) {
        background-color: green;
    }
    [popover] {
        visibility: hidden;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
