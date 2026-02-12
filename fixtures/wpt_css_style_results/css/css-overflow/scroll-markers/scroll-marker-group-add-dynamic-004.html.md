# css/css-overflow/scroll-markers/scroll-marker-group-add-dynamic-004.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-group-add-dynamic-004.html"
}
```

## style[0]

```css

  fieldset,
  legend {
    margin: 0;
    border: 0;
    padding: 0;
  }

  #scrollable::scroll-marker-group {
    display: block;
    height: 50px;
  }

  #scrollable.extra::scroll-marker-group {
    display: flex;
  }

  #scrollable>div::scroll-marker {
    display: block;
    width: 50px;
    height: 50px;
    content: "";
    background: green;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
