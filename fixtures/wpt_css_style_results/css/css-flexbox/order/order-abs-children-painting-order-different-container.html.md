# css/css-flexbox/order/order-abs-children-painting-order-different-container.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/order/order-abs-children-painting-order-different-container.html"
}
```

## style[0]

```css

  #test {
    display: flex;
    height: 100px;
    width: 100px;
  }

  #test2 {
    display: flex;
    height: 100px;
    width: 100px;
  }

  #order-1 {
    order: 1;
  }

  #order-2 {
    order: 2;
  }

  .inner {
    position: relative;
    width: 100px;
    height: 100px;
  }

  #order-1 .inner {
    background-color: green;
  }

  #order-2 .inner {
    background-color: red;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
