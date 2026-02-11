# css/css-multicol/fixed-in-nested-multicol.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/fixed-in-nested-multicol.html"
}
```

## style[0]

```css

  .multicol {
    column-count: 2;
    column-fill: auto;
    column-gap: 0px;
  }
  #outer {
    height: 100px;
    width: 100px;
    transform: translateX(0);
  }
  #inner {
    width: 50px;
  }
  .rel {
    position: relative;
  }
  .abs {
    position: absolute;
  }
  .fixed {
    position: fixed;
    height: 100px;
    width: 100px;
    top: -100px;
    background:green;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
