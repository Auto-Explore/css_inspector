# css/css-backgrounds/border-image-003.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-image-003.html"
}
```

## style[0]

```css

  div#red-overlapped-ref
    {
      background-color: red;
      height: 100px;
      width: 100px;
    }

  div#green-overlapping-test
    {
      border: red solid 40px;
      height: 20px;

      border-image-slice: 5 fill;
      border-image-source: url("support/12x12-green.png");
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
