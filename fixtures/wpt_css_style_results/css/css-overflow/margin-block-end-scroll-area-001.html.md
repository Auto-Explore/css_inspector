# css/css-overflow/margin-block-end-scroll-area-001.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/margin-block-end-scroll-area-001.html"
}
```

## style[0]

```css

  div
    {
      height: 200px;
    }

  div#test
    {
      font-size: 100px;
      /*
      Setting a font-size of 100px is to influence the
      margin on the P without setting it directly.
      */
      overflow: hidden;
    }

  div#red
    {
      background-color: red;
    }

  div#red > p
    {
      height: 1px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
