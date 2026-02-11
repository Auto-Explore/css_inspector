# css/mediaqueries/mq-calc-sign-function-003.html

```json
{
  "format_version": 3,
  "file": "css/mediaqueries/mq-calc-sign-function-003.html"
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

  @media screen and (aspect-ratio > calc(sign(17px - 1rem) * 59) / calc(79 * sign(17px - 1rem))) {
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
