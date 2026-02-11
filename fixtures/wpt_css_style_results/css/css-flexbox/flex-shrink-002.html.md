# css/css-flexbox/flex-shrink-002.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flex-shrink-002.html"
}
```

## style[0]

```css

  #container {
    background-color: red;
    display: flex;
    height: 100px;
    width: 100px;
  }
  #container div {
    height: 100px;
    width: 100px;
  }
  #test1 {
    flex-shrink: -2;
  }
  #test2 {
    background-color: green;
    flex-shrink: -3;
  }
  #cover {
    background-color: green;
    height: 100px;
    margin-top: -100px;
    width: 50px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
