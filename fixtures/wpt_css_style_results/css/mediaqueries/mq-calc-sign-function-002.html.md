# css/mediaqueries/mq-calc-sign-function-002.html

```json
{
  "format_version": 3,
  "file": "css/mediaqueries/mq-calc-sign-function-002.html"
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

  @media (width > calc(-1px * sign(15px - 1rem))) {
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
