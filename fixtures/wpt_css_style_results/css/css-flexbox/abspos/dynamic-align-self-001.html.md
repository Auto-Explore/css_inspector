# css/css-flexbox/abspos/dynamic-align-self-001.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/abspos/dynamic-align-self-001.html"
}
```

## style[0]

```css

  #reference-overlapped-red {
    position: absolute;
    background-color: red;
    width: 100px;
    height: 100px;
    z-index: -1;
  }

  #parent {
    position: relative;
    display: flex;
    width: 200px;
    height: 200px;
  }

  #child {
    display: flex;
    position: absolute;
    width: 100px;
    height: 100px;
    background-color: green;
    align-self: end;
  }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
