# css/css-contain/contain-paint-clip-018.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-paint-clip-018.html"
}
```

## style[0]

```css

  div
    {
      background-color: green;
      border-radius: 100px;
      contain: paint;
      height: 50px;
      padding: 25px;
      width: 50px;
    }

  div::after
    {
      background-color: red;
      content: "";
      display: block;
      height: 18px;
      margin-left: -29px;
      margin-top: -28px;
      width: 18px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
