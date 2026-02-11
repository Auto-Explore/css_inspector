# css/css-position/position-absolute-chrome-bug-001.html

```json
{
  "format_version": 3,
  "file": "css/css-position/position-absolute-chrome-bug-001.html"
}
```

## style[0]

```css

  #container {
    position: relative;
    border: 1px solid black;
  }
  .narrow {
    width: 200px;
    height: 300px;
  }
  .wide {
    width: 300px;
    height: 200px;
  }
  #target {
    background: green;
    position: absolute;
    width: 50px;
    height: 30px;
    left: 50%;
    top: 50%;
    margin-left: -25px;
    margin-top: -15px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
