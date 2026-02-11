# css/mediaqueries/mq-calc-sign-function-006.html

```json
{
  "format_version": 3,
  "file": "css/mediaqueries/mq-calc-sign-function-006.html"
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
    background-color: green;
  }

  @media (grid: calc(2 * sign(17px - 1rem))) {
    div {
      background-color: red;
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
