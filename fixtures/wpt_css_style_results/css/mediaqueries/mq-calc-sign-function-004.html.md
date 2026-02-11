# css/mediaqueries/mq-calc-sign-function-004.html

```json
{
  "format_version": 3,
  "file": "css/mediaqueries/mq-calc-sign-function-004.html"
}
```

## style[0]

```css

  :root {
    font-size: 16px;
  }
  div {
    width: 100px;
    height: 100px;
    background-color: red;
  }

  @media (resolution > calc(-1dppx * sign(17px - 1rem))) {
    div {
      background-color: green;
    }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
