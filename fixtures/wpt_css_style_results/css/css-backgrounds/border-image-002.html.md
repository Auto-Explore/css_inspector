# css/css-backgrounds/border-image-002.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-image-002.html"
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

      border-image-slice: 1;
      border-image-source: url("support/1pxgreen-98pxred.png");
    }

  div#green-filled-center
    {
      background-color: green;
      height: 20px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
