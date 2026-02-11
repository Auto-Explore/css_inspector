# css/css-anchor-position/reference/anchor-scroll-fixedpos-004-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/reference/anchor-scroll-fixedpos-004-ref.html"
}
```

## style[0]

```css

  body {
    margin: 0;
  }

  div {
    width: 100px;
    height: 100px;
  }

  #anchor {
    margin-top: 105vh;
    background: orange;
  }

  #anchored {
    position: absolute;
    top: calc(105vh - 100px);
    left: 100px;
    background: green;

    transform: scale(2);
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
