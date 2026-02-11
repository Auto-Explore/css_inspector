# css/mediaqueries/mq-calc-sign-function-005.html

```json
{
  "format_version": 3,
  "file": "css/mediaqueries/mq-calc-sign-function-005.html"
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

  @media (grid: calc(2 * sign(16px - 1rem))) {
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
