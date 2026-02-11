# css/css-flexbox/flex-grow-007.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flex-grow-007.html"
}
```

## style[0]

```css

  .container {
    background: red;
    height: 50px;
    width: 100px;
  }
  #test1, #test2 {
    display: flex;
    width: 190px;
  }
  #test1 > div, #test2 > div {
    background: green;
    height: 50px;
  }
  #test1 > div {
    flex-grow: 0.1;
    width: 90px;
  }
  #test2 > div {
    flex-grow: 0.05;
    width: 45px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
